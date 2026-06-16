//! Simulates an E002 (Unclosed Environment) diagnostic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report, Result};

fn main() -> Result<()> {
    // 1. The LaTeX source with an unclosed \begin{figure}
    let source = r"\documentclass{article}
\begin{document}
Some text here.
\begin{figure}
This figure is never closed!
\end{document}"
        .to_string();

    // 2. Initialize the sink
    let mut sink = LtxDiagnosticSink::new();

    // 3. Dynamically find the byte offset of \begin{figure}
    if let Some(start) = source.find(r"\begin{figure}") {
        let end = start + r"\begin{figure}".len();
        let span = LtxSpan::new(start, end, "main.tex");

        // 4. Create the E002 diagnostic placeholder
        let diag = LtxDiagnostic::UnclosedEnvironment {
            name: "figure".to_string(),
            open_span: (0, 0).into(),       // Placeholder, overwritten by with_source
            src: NamedSource::new("main.tex", source.clone()),   // Placeholder, overwritten by with_source
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
