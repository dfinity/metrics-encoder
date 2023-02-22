use super::{validate_prometheus_name, MetricsEncoder};

fn as_string(encoder: MetricsEncoder<Vec<u8>>) -> String {
    String::from_utf8(encoder.into_inner()).unwrap()
}

#[test]
fn test_labeled_counter_metrics() {
    let mut encoder = MetricsEncoder::new(vec![0u8; 0], 1395066363000);
    encoder
        .counter_vec("http_requests_total", "The total number of HTTP requests.")
        .unwrap()
        .value(&[("method", "post"), ("code", "200")], 1027.0)
        .unwrap()
        .value(&[("method", "post"), ("code", "400")], 3.0)
        .unwrap();

    assert_eq!(
        r#"# HELP http_requests_total The total number of HTTP requests.
# TYPE http_requests_total counter
http_requests_total{method="post",code="200"} 1027 1395066363000
http_requests_total{method="post",code="400"} 3 1395066363000
"#,
        as_string(encoder)
    );
}

#[test]
fn test_labeled_gauge_metrics() {
    let mut encoder = MetricsEncoder::new(vec![0u8; 0], 1395066363000);
    encoder
        .gauge_vec("cpu_temperature", "CPU temperature in celsius.")
        .unwrap()
        .value(&[("core", "1")], 40.0)
        .unwrap()
        .value(&[("core", "2")], 43.0)
        .unwrap();

    assert_eq!(
        r#"# HELP cpu_temperature CPU temperature in celsius.
# TYPE cpu_temperature gauge
cpu_temperature{core="1"} 40 1395066363000
cpu_temperature{core="2"} 43 1395066363000
"#,
        as_string(encoder)
    );
}

#[test]
fn test_histogram() {
    let mut encoder = MetricsEncoder::new(vec![0u8; 0], 1395066363000);
    encoder
        .encode_histogram(
            "http_request_duration_seconds",
            [
                (0.05, 24054f64),
                (0.1, 33444.0 - 24054.0),
                (0.2, 100392.0 - 33444.0),
                (0.5, 129389.0 - 100392.0),
                (1.0, 133988.0 - 129389.0),
                (std::f64::INFINITY, 144320.0 - 133988.0),
            ]
            .into_iter(),
            53423.0,
            "A histogram of the request duration.",
        )
        .unwrap();

    assert_eq!(
        r#"# HELP http_request_duration_seconds A histogram of the request duration.
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.05"} 24054 1395066363000
http_request_duration_seconds_bucket{le="0.1"} 33444 1395066363000
http_request_duration_seconds_bucket{le="0.2"} 100392 1395066363000
http_request_duration_seconds_bucket{le="0.5"} 129389 1395066363000
http_request_duration_seconds_bucket{le="1"} 133988 1395066363000
http_request_duration_seconds_bucket{le="+Inf"} 144320 1395066363000
http_request_duration_seconds_sum 53423 1395066363000
http_request_duration_seconds_count 144320 1395066363000
"#,
        as_string(encoder)
    );
}

#[test]
fn test_histogram_no_infinity() {
    let mut encoder = MetricsEncoder::new(vec![0u8; 0], 1395066363000);
    encoder
        .encode_histogram(
            "http_request_duration_seconds",
            [
                (0.05, 24054f64),
                (0.1, 33444.0 - 24054.0),
                (0.2, 100392.0 - 33444.0),
                (0.5, 129389.0 - 100392.0),
                (1.0, 133988.0 - 129389.0),
                (2.0, 144320.0 - 133988.0),
            ]
            .into_iter(),
            53423.0,
            "A histogram of the request duration.",
        )
        .unwrap();

    assert_eq!(
        r#"# HELP http_request_duration_seconds A histogram of the request duration.
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.05"} 24054 1395066363000
http_request_duration_seconds_bucket{le="0.1"} 33444 1395066363000
http_request_duration_seconds_bucket{le="0.2"} 100392 1395066363000
http_request_duration_seconds_bucket{le="0.5"} 129389 1395066363000
http_request_duration_seconds_bucket{le="1"} 133988 1395066363000
http_request_duration_seconds_bucket{le="2"} 144320 1395066363000
http_request_duration_seconds_bucket{le="+Inf"} 144320 1395066363000
http_request_duration_seconds_sum 53423 1395066363000
http_request_duration_seconds_count 144320 1395066363000
"#,
        as_string(encoder)
    );
}

#[test]
fn test_histogram_vec() {
    let mut encoder = MetricsEncoder::new(vec![0u8; 0], 1395066363000);
    encoder
        .histogram_vec(
            "http_request_duration_seconds",
            "A histogram of the request duration.",
        )
        .unwrap()
        .histogram(
            &[("backend", "canary")],
            [
                (0.05, 24054f64),
                (0.1, 33444.0 - 24054.0),
                (0.2, 100392.0 - 33444.0),
                (0.5, 129389.0 - 100392.0),
                (1.0, 133988.0 - 129389.0),
                (std::f64::INFINITY, 144320.0 - 133988.0),
            ]
            .into_iter(),
            53423.0,
        )
        .unwrap()
        .histogram(
            &[("backend", "stable")],
            [
                (0.05, 24054f64),
                (0.1, 33444.0 - 24054.0),
                (0.2, 100392.0 - 33444.0),
                (0.5, 129389.0 - 100392.0),
                (1.0, 133988.0 - 129389.0),
                (std::f64::INFINITY, 144320.0 - 133988.0),
            ]
            .into_iter(),
            53423.0,
        )
        .unwrap();

    assert_eq!(
        r#"# HELP http_request_duration_seconds A histogram of the request duration.
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{backend="canary",le="0.05"} 24054 1395066363000
http_request_duration_seconds_bucket{backend="canary",le="0.1"} 33444 1395066363000
http_request_duration_seconds_bucket{backend="canary",le="0.2"} 100392 1395066363000
http_request_duration_seconds_bucket{backend="canary",le="0.5"} 129389 1395066363000
http_request_duration_seconds_bucket{backend="canary",le="1"} 133988 1395066363000
http_request_duration_seconds_bucket{backend="canary",le="+Inf"} 144320 1395066363000
http_request_duration_seconds_sum{backend="canary"} 53423 1395066363000
http_request_duration_seconds_count{backend="canary"} 144320 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="0.05"} 24054 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="0.1"} 33444 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="0.2"} 100392 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="0.5"} 129389 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="1"} 133988 1395066363000
http_request_duration_seconds_bucket{backend="stable",le="+Inf"} 144320 1395066363000
http_request_duration_seconds_sum{backend="stable"} 53423 1395066363000
http_request_duration_seconds_count{backend="stable"} 144320 1395066363000
"#,
        as_string(encoder)
    );
}

#[test]
#[should_panic(expected = "Empty names are not allowed")]
fn validate_empty_name() {
    validate_prometheus_name("")
}

#[test]
#[should_panic(expected = "Name '⇒Γ' does not match pattern [a-zA-Z_][a-zA-Z0-9_]")]
fn validate_unicode_name() {
    validate_prometheus_name("⇒Γ")
}

#[test]
#[should_panic(expected = "Name 'http:rule' does not match pattern [a-zA-Z_][a-zA-Z0-9_]")]
fn validate_rule_name() {
    validate_prometheus_name("http:rule")
}
