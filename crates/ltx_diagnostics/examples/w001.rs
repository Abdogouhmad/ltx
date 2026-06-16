//! Simulates a W001 (Deprecated Math Delimiter) diagnostic

use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report};

fn main() -> miette::Result<()> {
    // 1. The LaTeX source with a mistake
    let source = r"\documentclass{article}
\begin{document}
Hello \foo world!
This is $$deprecated$$ math.
\end{document}"
        .to_string();

    // 2. Create a sink to collect our issues
    let mut sink = LtxDiagnosticSink::new();
    // 3. dynamic find the byte offsets of $$ and report as deprecated math
    if let Some(offset) = source.find("$$") {
        let span = LtxSpan::new(offset, offset + 2, "main.tex");

        let diag = LtxDiagnostic::DeprecatedMathDelimiter {
            span: (offset, offset + 2).into(),
            src: NamedSource::new("main.tex", source.clone()),
        }
        .with_source(span, source.clone(), "main.tex".to_string());
        sink.push(diag);
    }
    // 4. render output
    println!("🔍 Found {} issue(s):\n", sink.len());

    for diag in sink.drain_sorted() {
        let r = Report::from(diag);
        println!("{:?}", r);
    }
    Ok(())
}
