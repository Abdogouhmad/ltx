#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

use std::borrow::Cow;
use std::sync::Arc;

use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxError, LtxFileId, LtxSeverity, LtxSourceMap, LtxSpan,
    render_json_into,
};
use pretty_assertions::assert_eq;

fn make_source_map(source: &str) -> (LtxFileId, LtxSourceMap) {
    let mut map = LtxSourceMap::new();
    let id = map.add_inline("test.tex", source);
    (id, map)
}

#[test]
fn test_line_col_first_line() {
    let (file_id, map) = make_source_map("hello");
    let lc = map.line_col(file_id, 0);
    assert_eq!(lc, Some((1, 1)));
}

#[test]
fn test_line_col_second_line() {
    let (file_id, map) = make_source_map("hello\nworld");
    let offset = "hello\n".len();
    let lc = map.line_col(file_id, offset);
    assert_eq!(lc, Some((2, 1)));
}

#[test]
fn test_line_col_middle_of_line() {
    let (file_id, map) = make_source_map("abcdef\nghijk");
    let lc = map.line_col(file_id, 3);
    assert_eq!(lc, Some((1, 4)));
}

#[test]
fn test_line_col_empty_source() {
    let mut map = LtxSourceMap::new();
    let id = map.add_inline("empty.tex", "");
    let lc = map.line_col(id, 0);
    assert_eq!(lc, Some((1, 1)));
}

#[test]
fn test_span_merge_overlapping() {
    let fid = LtxFileId(0);
    let a = LtxSpan::new(0, 5, fid);
    let b = LtxSpan::new(3, 8, fid);
    let merged = a.merge(&b);
    assert_eq!(merged.start, 0);
    assert_eq!(merged.end, 8);
    assert_eq!(merged.file_id, fid);
}

#[test]
fn test_span_merge_disjoint() {
    let fid = LtxFileId(1);
    let a = LtxSpan::new(0, 2, fid);
    let b = LtxSpan::new(10, 15, fid);
    let merged = a.merge(&b);
    assert_eq!(merged.start, 0);
    assert_eq!(merged.end, 15);
}

#[test]
fn test_span_len() {
    let span = LtxSpan::new(5, 12, LtxFileId(0));
    assert_eq!(span.len(), 7);
}

#[test]
fn test_span_is_empty() {
    let span = LtxSpan::new(4, 4, LtxFileId(0));
    assert!(span.is_empty());

    let non_empty = LtxSpan::new(0, 1, LtxFileId(0));
    assert!(!non_empty.is_empty());
}

#[test]
fn test_drain_sorted_errors_first() {
    let (fid, source_map) = make_source_map("hello world");
    let source_map = Arc::new(source_map);

    let mut sink = LtxDiagnosticSink::new();

    sink.push(LtxDiagnostic::new(
        LtxError::UndefinedReference {
            key: Cow::Borrowed("fig:one"),
            span: LtxSpan::new(0, 5, fid),
        },
        Arc::clone(&source_map),
    ));

    sink.push(LtxDiagnostic::new(
        LtxError::UnexpectedToken {
            found: Cow::Borrowed("@"),
            span: LtxSpan::new(6, 7, fid),
        },
        Arc::clone(&source_map),
    ));

    sink.push(LtxDiagnostic::new(
        LtxError::UnmatchedBrace {
            found: Cow::Borrowed("}"),
            span: LtxSpan::new(8, 9, fid),
        },
        Arc::clone(&source_map),
    ));

    let sorted = sink.drain_sorted();
    assert_eq!(sorted.len(), 3);

    let severities: Vec<_> = sorted.iter().map(LtxDiagnostic::severity).collect();
    assert_eq!(severities[0], LtxSeverity::Error);
    assert_eq!(severities[1], LtxSeverity::Error);
    assert_eq!(severities[2], LtxSeverity::Warning);
}

#[test]
fn test_render_json_into() {
    let (fid, source_map) = make_source_map("hello");
    let source_map = Arc::new(source_map);

    let diag = LtxDiagnostic::new(
        LtxError::UnexpectedToken {
            found: Cow::Borrowed("!"),
            span: LtxSpan::new(0, 1, fid),
        },
        Arc::clone(&source_map),
    );

    let mut buf = Vec::new();
    render_json_into(&[diag], &mut buf).expect("render_json_into should succeed");

    let json_str = String::from_utf8(buf).expect("valid utf8");
    let parsed: serde_json::Value = serde_json::from_str(&json_str).expect("valid json");

    assert!(parsed.is_array());
    let arr = parsed.as_array().expect("array");
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["severity"], "error");
    assert_eq!(arr[0]["code"], "LTX::E001");
}

#[test]
fn test_source_map_add_inline() {
    let mut map = LtxSourceMap::new();
    assert!(map.is_empty());

    let id = map.add_inline("a.tex", "source a");
    assert_eq!(map.len(), 1);
    assert!(!map.is_empty());

    let file = map.get_file(id).expect("file should exist");
    assert_eq!(file.id(), id);
    assert_eq!(file.path().to_str(), Some("a.tex"));
}

#[test]
fn test_source_map_multiple_files() {
    let mut map = LtxSourceMap::new();

    let id1 = map.add_inline("first.tex", "content one");
    let id2 = map.add_inline("second.tex", "content two");

    assert_eq!(map.len(), 2);
    assert_ne!(id1, id2);

    let f1 = map.get_file(id1).expect("first file");
    let f2 = map.get_file(id2).expect("second file");
    assert_eq!(f1.path().to_str(), Some("first.tex"));
    assert_eq!(f2.path().to_str(), Some("second.tex"));
}
