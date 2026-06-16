//! Simulates an E003 (Missing DocumentClass) diagnostic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report, Result};

fn main() -> Result<()> {
    // 1. Source WITHOUT \documentclass (the actual trigger for E003)
    let source = r"\begin{document}
Some text here.
\end{document}"
        .to_string();

    // 2. Initialize the sink
    let mut sink = LtxDiagnosticSink::new();

    // 3. Point to the start of the file where \documentclass SHOULD be
    let span = LtxSpan::new(0, 0, "main.tex");

    // 4. Create the E003 diagnostic placeholder
    let diag = LtxDiagnostic::MissingDocumentClass {
        span: (0, 0).into(),
        src: NamedSource::new("main.tex".to_string(), source.clone()),
    }.with_source(span, source.clone(), "main.tex".to_string());

    sink.push(diag);

    // 5. Render the output
    println!("🔍 Found {} issue(s):\n", sink.len());

    for diag in sink.drain_sorted() {
        let report = Report::new(diag);
        println!("{:?}\n", report);
    }

    Ok(())
}
