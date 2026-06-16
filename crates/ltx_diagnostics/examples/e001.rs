//! Simulates an E001 (Unexpected Token) diagnostic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report, Result};

fn main() -> Result<()> {
    // 1. The LaTeX source with a syntax error
    let source = r"\documentclass{article}
\begin{document}
Hello } world!
\end{document}"
        .to_string();

    // 2. Initialize the sink
    let mut sink = LtxDiagnosticSink::new();

    // 3. Dynamically find the byte offset of the unexpected '}'
    if let Some(start) = source.find("}") {
        let end = start + 1; // '}' is 1 byte long
        let span = LtxSpan::new(start, end, "main.tex");

        // 4. Create the diagnostic placeholder
        let diag = LtxDiagnostic::UnexpectedToken {
            found: "}".to_string(),
            span: (0, 0).into(),       // Placeholder, overwritten by with_source
            src: NamedSource::new("main.tex", source.clone()), // Placeholder, overwritten by with_source
        }
        .with_source(span, source.clone(), "main.tex".to_string());

        sink.push(diag);
    }

    // 5. Render the output
    println!("🔍 Found {} issue(s):\n", sink.len());

    for diag in sink.drain_sorted() {
        let report = Report::new(diag);
        println!("{:?}\n", report);
    }

    Ok(())
}
