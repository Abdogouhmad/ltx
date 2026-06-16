//! Tests for LtxDiagnosticSink collection logic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan, LtxSeverity};
use miette::NamedSource;

fn make_error() -> LtxDiagnostic {
    LtxDiagnostic::UnknownCommand {
        name: "bad".into(),
        span: (0, 0).into(),
        src: NamedSource::new("", "".to_string()),
    }
    .with_source(LtxSpan::new(0, 4, "t.tex"), "\\bad".into(), "t.tex".into())
}

fn make_warning() -> LtxDiagnostic {
    LtxDiagnostic::TrailingWhitespace {
        span: (0, 0).into(),
        src: NamedSource::new("", "".to_string()),
    }
    .with_source(LtxSpan::new(0, 1, "t.tex"), " ".into(), "t.tex".into())
}

#[test]
fn test_sink_collects_multiple() {
    let mut sink = LtxDiagnosticSink::new();
    sink.push(make_error());
    sink.push(make_warning());

    assert_eq!(sink.len(), 2);
    assert!(!sink.is_empty());
}

#[test]
fn test_has_errors_only_checks_errors() {
    let mut sink = LtxDiagnosticSink::new();
    sink.push(make_warning());
    assert!(!sink.has_error());

    sink.push(make_error());
    assert!(sink.has_error());
}

#[test]
fn test_drain_sorted_errors_first() {
    let mut sink = LtxDiagnosticSink::new();
    sink.push(make_warning());
    sink.push(make_error());
    sink.push(make_warning());

    let sorted = sink.drain_sorted();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].severity(), LtxSeverity::Error);
    assert_eq!(sorted[1].severity(), LtxSeverity::Warning);
    assert_eq!(sorted[2].severity(), LtxSeverity::Warning);
}

#[test]
fn test_get_by_severity_filters_correctly() {
    let mut sink = LtxDiagnosticSink::new();
    sink.push(make_error());
    sink.push(make_warning());
    sink.push(make_error());

    let errors = sink.get_by_severity(LtxSeverity::Error);
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().all(|d| d.severity() == LtxSeverity::Error));
}

#[test]
fn test_into_diagnostics_consumes_sink() {
    let mut sink = LtxDiagnosticSink::new();
    sink.push(make_error());

    let diags = sink.into_diagnostics();
    assert_eq!(diags.len(), 1);
    // sink is moved, cannot be used here — this is intentional
}
