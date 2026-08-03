//! small test for diagnostic error codes

use std::borrow::Cow;
use std::sync::Arc;

use thiserror::Error as ThisError;

use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticSink, LtxDiagnosticSource, LtxFileId, LtxSourceMap, LtxSpan,
};
use miette::Diagnostic as _;

use pretty_assertions::assert_eq as pretty_eq;

/// Minimal stand-in for a crate-owned error type (the diagnostics crate
/// itself defines no domain errors — see crate docs).
#[derive(Debug, ThisError, miette::Diagnostic, Clone)]
enum TestError {
    #[error("undefined control sequence `\\{name}`")]
    #[diagnostic(code(LTX::PARSER::E100), severity(Error))]
    UndefinedControlSequence {
        name: Cow<'static, str>,
        #[label("undefined control sequence")]
        span: LtxSpan,
    },

    #[error("unexpected token `{found}`")]
    #[diagnostic(code(LTX::LEXER::E001), severity(Error))]
    UnexpectedToken {
        found: Cow<'static, str>,
        #[label("unexpected token")]
        span: LtxSpan,
    },

    #[error("unmatched brace detected: `{found}`")]
    #[diagnostic(code(LTX::LEXER::E003), severity(Error))]
    UnmatchedBrace {
        found: Cow<'static, str>,
        #[label("unmatched brace")]
        span: LtxSpan,
    },

    #[error("mismatched environment: expected `\\end{{{expected}}}`, found `\\end{{{found}}}`")]
    #[diagnostic(code(LTX::PARSER::E101), severity(Error))]
    MismatchedEnvironment {
        expected: Cow<'static, str>,
        found: Cow<'static, str>,
        #[label("mismatched \\end")]
        span: LtxSpan,
    },
}

impl LtxDiagnosticSource for TestError {
    fn span(&self) -> LtxSpan {
        match self {
            Self::UndefinedControlSequence { span, .. }
            | Self::UnexpectedToken { span, .. }
            | Self::UnmatchedBrace { span, .. }
            | Self::MismatchedEnvironment { span, .. } => *span,
        }
    }
}

fn init_filemapped() -> (LtxFileId, LtxSourceMap) {
    let source_text = r"hello, world! \nocommand";
    let file_name = "example.tex";
    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_inline(file_name, source_text);
    (file_id, source_map)
}

fn make_diag(error: TestError) -> LtxDiagnostic {
    let source_text = r"hello, world! \nocommand";
    let mut source_map = LtxSourceMap::new();
    let _file_id = source_map.add_inline("example.tex", source_text);
    LtxDiagnostic::new(error, Arc::new(source_map))
}

/// Helper: extract the error code string from a diagnostic.
fn diag_code(diag: &LtxDiagnostic) -> String {
    diag.code()
        .map_or_else(|| "<none>".to_string(), |c| c.to_string())
}

#[test]
fn test_sink_len_and_has_error() {
    let (file_id, source_map) = init_filemapped();
    let source_map = Arc::new(source_map);
    let mut sink = LtxDiagnosticSink::new();

    sink.push(LtxDiagnostic::new(
        TestError::UndefinedControlSequence {
            name: Cow::Borrowed("nocommand"),
            span: LtxSpan::new(33, 43, file_id),
        },
        Arc::clone(&source_map),
    ));

    pretty_eq!(sink.len(), 1);
    pretty_eq!(sink.has_error(), true);
}

// ── Error code matching ──────────────────────────────────────────────

#[test]
fn code_e100_undefined_control_sequence() {
    let diag = make_diag(TestError::UndefinedControlSequence {
        name: Cow::Borrowed("nocommand"),
        span: LtxSpan::new(0, 10, LtxFileId(0)),
    });
    pretty_eq!(diag_code(&diag), "LTX::PARSER::E100");
}

#[test]
fn code_e001_unexpected_token() {
    let diag = make_diag(TestError::UnexpectedToken {
        found: Cow::Borrowed("@"),
        span: LtxSpan::new(0, 1, LtxFileId(0)),
    });
    pretty_eq!(diag_code(&diag), "LTX::LEXER::E001");
}

#[test]
fn code_e003_unmatched_brace() {
    let diag = make_diag(TestError::UnmatchedBrace {
        found: Cow::Borrowed("}"),
        span: LtxSpan::new(5, 6, LtxFileId(0)),
    });
    pretty_eq!(diag_code(&diag), "LTX::LEXER::E003");
}

#[test]
fn code_e101_mismatched_environment() {
    let diag = make_diag(TestError::MismatchedEnvironment {
        expected: Cow::Borrowed("document"),
        found: Cow::Borrowed("documen"),
        span: LtxSpan::new(0, 20, LtxFileId(0)),
    });
    pretty_eq!(diag_code(&diag), "LTX::PARSER::E101");
}
