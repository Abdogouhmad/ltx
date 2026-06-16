//! Tests for Severity enum behavior
use ltx_diagnostics::LtxSeverity;

#[test]
fn test_severity_ordering() {
    // Errors should be "greater than" warnings for sorting
    assert!(LtxSeverity::Error > LtxSeverity::Warning);
    assert!(LtxSeverity::Warning > LtxSeverity::Hint);
    assert!(LtxSeverity::Error > LtxSeverity::Hint);
}

#[test]
fn test_is_error() {
    assert!(LtxSeverity::Error.is_error());
    assert!(!LtxSeverity::Warning.is_error());
    assert!(!LtxSeverity::Hint.is_error());
}

#[test]
fn test_to_miette_bridge() {
    use miette::Severity as MietteSev;

    assert_eq!(LtxSeverity::Error.to_miette(), MietteSev::Error);
    assert_eq!(LtxSeverity::Warning.to_miette(), MietteSev::Warning);
    assert_eq!(LtxSeverity::Hint.to_miette(), MietteSev::Advice);
}

#[test]
fn test_severity_serde_lowercase() {
    // Verify serde produces lowercase strings
    let json = serde_json::to_string(&LtxSeverity::Error).unwrap();
    assert_eq!(json, r#""error""#);

    let json_warn = serde_json::to_string(&LtxSeverity::Warning).unwrap();
    assert_eq!(json_warn, r#""warning""#);
}
