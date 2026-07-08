#[test]
fn parse_valid_log() {
    let logs = parse_log_file("tests/valid_log.json").unwrap();

    assert_eq!(logs.len(), 1);

    assert_eq!(logs[0].level, "info");

    assert!(logs[0].message.contains("Backend"));
}

#[test]
fn parse_simple_message() {
    let logs = parse_log_file("tests/valid_log.json").unwrap();

    assert_eq!(logs[0].message, "Backend started");
}

#[test]
fn clean_ansi() {
    let text = "\u{001b}[32mINFO\u{001b}[39m";

    let cleaned = clean_message(text);

    assert_eq!(cleaned, "INFO");
}

#[test]
fn format_timestamp() {
    let result = format_timestamp(1776414846757);

    assert!(!result.is_empty());

    assert!(result.contains('/'));
}

#[test]
fn missing_file() {
    let result = parse_log_file("tests/inexistant.json");

    assert!(result.is_err());
}

#[test]
fn empty_file() {
    let logs = parse_log_file("tests/empty_log.json").unwrap();

    assert!(logs.is_empty());
}

#[test]
fn invalid_json() {
    let logs = parse_log_file("tests/invalid_log.json").unwrap();

    assert!(logs.is_empty());
}
