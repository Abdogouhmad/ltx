//! Tests for LtxSpan byte-range logic
use ltx_diagnostics::LtxSpan;
use pretty_assertions::assert_eq;

#[test]
fn test_span_merge_overlapping() {
    let s1 = LtxSpan::new(10, 25, "main.tex");
    let s2 = LtxSpan::new(20, 35, "main.tex");
    let merged = s1.merge(&s2);

    assert_eq!(merged.start, 10);
    assert_eq!(merged.end, 35);
    assert_eq!(merged.file.as_ref(), "main.tex");
}

#[test]
fn test_span_merge_adjacent() {
    let s1 = LtxSpan::new(0, 10, "main.tex");
    let s2 = LtxSpan::new(10, 20, "main.tex");
    let merged = s1.merge(&s2);

    assert_eq!(merged.start, 0);
    assert_eq!(merged.end, 20);
}

#[test]
fn test_span_to_miette_conversion() {
    let span = LtxSpan::new(42, 50, "test.tex");
    let miette: miette::SourceSpan = span.into();

    assert_eq!(miette.offset(), 42);
    assert_eq!(miette.len(), 8);
}

#[test]
fn test_span_len_and_empty() {
    let s = LtxSpan::new(5, 5, "f.tex");
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);

    let s2 = LtxSpan::new(5, 10, "f.tex");
    assert!(!s2.is_empty());
    assert_eq!(s2.len(), 5);
}
