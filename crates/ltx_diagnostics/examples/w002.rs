//! Simulates a W002 (Trailing Whitespace) diagnostic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report, Result};

fn main() -> Result<()> {
    // 1. Source with intentional trailing spaces after "Hello" and "World"
    let source = format!(
        r"\documentclass{{article}}
    \begin{{document}}
    Hello{}
    World{}
    \end{{document}}",
        "\x20\x20\x20", // 3 spaces after Hello
        "\x20\x20"      // 2 spaces after World
    );

    // 2. Initialize the sink
    let mut sink = LtxDiagnosticSink::default();

    // 3. Find ALL instances of trailing whitespace
    // In production, your linter would iterate over lines and check each one
    let lines: Vec<(usize, &str)> = source.lines().enumerate().collect();

    for (_, line) in &lines {
        let trimmed_len = line.trim_end().len();
        if trimmed_len < line.len() {
            // Calculate byte offset of the trailing whitespace
            let line_start = source[..line.as_ptr() as usize - source.as_ptr() as usize].len();
            let ws_start = line_start + trimmed_len;
            let ws_end = line_start + line.len();

            let span = LtxSpan::new(ws_start, ws_end, "main.tex");

            let diag = LtxDiagnostic::TrailingWhitespace {
                span: (0, 0).into(),       // Placeholder
                src: NamedSource::new("main.tex", source.clone()),
            }
            .with_source(span, source.clone(), "main.tex".to_string());

            sink.push(diag);
        }
    }

    // 4. Render the output
    println!("🔍 Found {} issue(s):\n", sink.len());

    for diag in sink.drain_sorted() {
        let report = Report::new(diag);
        println!("{:?}\n", report);
    }

    Ok(())
}
