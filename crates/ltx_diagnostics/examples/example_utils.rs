use ltx_diagnostics::{LtxDiagnostic, LtxDiagnosticSink, LtxSpan};
use miette::Report;

#[allow(unreachable_pub)]
pub struct ExampleRunner {
    pub file_name: String,
    pub source: String,
}

impl ExampleRunner {
    #[allow(unreachable_pub)]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            file_name: "main.tex".to_string(),
            source: source.into(),
        }
    }

    #[allow(unreachable_pub, clippy::print_stdout)]
    pub fn trigger_and_render<F, D>(&self, target_pattern: &str, make_diagnostic: F)
    where
        F: FnOnce(LtxSpan) -> D,
        D: Into<LtxDiagnostic>,
    {
        let mut sink = LtxDiagnosticSink::new();

        if let Some(start) = self.source.find(target_pattern) {
            let end = start + target_pattern.len();

            // 1. Create your custom crate's LtxSpan tracking coordinate
            let ltx_span = LtxSpan::new(start, end, self.file_name.clone());

            // 2. Instantiate the core inner variant
            let core_diagnostic: LtxDiagnostic = make_diagnostic(ltx_span.clone()).into();

            // 3. CORRECTED PATH: Pass the exact types your method expects!
            // under the hood, LtxDiagnostic::with_source converts ltx_span -> miette::SourceSpan
            // and bundles the file_name + source strings into the correct NamedSource.
            let contextualized_diag =
                core_diagnostic.with_source(ltx_span, self.source.clone(), self.file_name.clone());

            sink.push(contextualized_diag);
        } else {
            panic!(
                "Test setup error: Could not find substring '{target_pattern}' in the source mock string."
            );
        }

        println!("🔍 Found {} issue(s):\n", sink.len());
        for diag in sink.drain_sorted() {
            let report = Report::new(diag);
            println!("{report:?}\n");
        }
    }
}
