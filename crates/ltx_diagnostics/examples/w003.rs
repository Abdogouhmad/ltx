//! Simulates a W003 (Unreferenced Label) diagnostic
use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::{NamedSource, Report, Result};

fn main() -> Result<()> {
    // 1. Source with two labels, but only one is referenced
    let source = r"\documentclass{article}
\begin{document}
\label{foo}
Some text here.
See Section~\ref{foo}.
\label{bar}
More text without referencing bar.
\end{document}"
        .to_string();

    // 2. Initialize the sink
    let mut sink = LtxDiagnosticSink::default();

    // 3. Simulate linter logic: find all \label{} and \ref{} commands
    let mut defined_labels: Vec<(String, usize)> = Vec::new();
    let mut used_labels: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Find all \label{name} occurrences
    let label_regex = regex::Regex::new(r"\\label\{([^}]+)\}").unwrap();
    for cap in label_regex.captures_iter(&source) {
        let name = cap[1].to_string();
        let start = cap.get(0).unwrap().start();
        defined_labels.push((name, start));
    }

    // Find all \ref{name} occurrences
    let ref_regex = regex::Regex::new(r"\\ref\{([^}]+)\}").unwrap();
    for cap in ref_regex.captures_iter(&source) {
        used_labels.insert(cap[1].to_string());
    }

    // 4. Report unreferenced labels
    for (name, start_pos) in &defined_labels {
        if !used_labels.contains(name) {
            let end_pos = start_pos + format!(r"\label{{{}}}", name).len();
            let span = LtxSpan::new(*start_pos, end_pos, "main.tex");

            let diag = LtxDiagnostic::UnreferencedLabel {
                name: name.clone(),
                span: (0, 0).into(),       // Placeholder
                src: NamedSource::new("main.tex", source.clone()), // Placeholder
            }
            .with_source(span, source.clone(), "main.tex".to_string());

            sink.push(diag);
        }
    }

    // 5. Render the output
    println!("🔍 Found {} issue(s):\n", sink.len());

    for diag in sink.drain_sorted() {
        let report = Report::new(diag);
        println!("{:?}\n", report);
    }

    Ok(())
}
