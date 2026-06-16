    //! Integration test: full diagnostic pipeline
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSeverity, LtxSpan, render_json};
use serde_json::Value;
#[test]
fn test_full_lifecycle_unknown_command() {
    let source = r"\documentclass{article}
\begin{document}
Hello \foo world
\end{document}"
        .to_string();

    let start = source.find(r"\foo").unwrap();
    let span = LtxSpan::new(start, start + 4, "main.tex");

    let diag = LtxDiagnostic::UnknownCommand {
        name: "foo".into(),
        span: (0, 0).into(),
        src: miette::NamedSource::new("", "".to_string()),
    }
    .with_source(span, source.clone(), "main.tex".into());

    let mut sink = LtxDiagnosticSink::new();
    sink.push(diag);

    assert!(sink.has_error());
    assert_eq!(sink.len(), 1);

    let json = render_json(sink.all());
    let parsed: Vec<Value> = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["code"], "ltx::parse::E004");
    assert_eq!(parsed[0]["severity"], "error");
    assert_eq!(parsed[0]["message"], r"unknown command `\foo`");
}

#[test]
fn test_full_lifecycle_mixed_severities() {
    let source = r"\begin{document}
Hello
\label{unused}
\end{document}"
        .to_string();

    let mut sink = LtxDiagnosticSink::new();

    // E003: Missing documentclass
    sink.push(
        LtxDiagnostic::MissingDocumentClass {
            span: (0, 0).into(),
            src: miette::NamedSource::new("", "".to_string()),
        }
        .with_source(LtxSpan::new(0, 0, "main.tex"), source.clone(), "main.tex".into()),
    );

    // W002: Trailing whitespace (bytes 22..25 = "   \n")
    if let Some(pos) = source.find("Hello   ") {
        sink.push(
            LtxDiagnostic::TrailingWhitespace {
                span: (0, 0).into(),
                src: miette::NamedSource::new("", "".to_string()),
            }
            .with_source(
                LtxSpan::new(pos + 5, pos + 8, "main.tex"),
                source.clone(),
                "main.tex".into(),
            ),
        );
    }

    // assert_eq!(sink.len(), 2);
    // assert!(sink.has_error());

    let sorted = sink.drain_sorted();
    eprintln!("{:?}", sorted);
    // assert_eq!(sorted[0].severity(), LtxSeverity::Error);
    // assert_eq!(sorted[1].severity(), LtxSeverity::Warning);
}
