use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticInner, LtxDiagnosticSink, LtxSourceMap, LtxSpan, LtxFileId
};
use miette::Report;
use std::sync::Arc;

#[allow(unreachable_pub)]
pub struct ExampleRunner {
    pub source_map: Arc<LtxSourceMap>,
    pub file_id: LtxFileId,
}

impl ExampleRunner {
    #[allow(unreachable_pub)]
    pub fn new(source: impl Into<String>) -> Self {
        let mut source_map = LtxSourceMap::new();
        let file_id = source_map.add_inline("main.tex", source.into());
        Self {
            source_map: Arc::new(source_map),
            file_id,
        }
    }

    #[allow(unreachable_pub, clippy::print_stdout)]
    pub fn trigger_and_render<F, D>(&self, target_pattern: &str, make_diagnostic: F)
    where
        F: FnOnce(LtxSpan) -> D,
        D: Into<LtxDiagnosticInner>,
    {
        let mut sink = LtxDiagnosticSink::new();

        let file = self.source_map.get_file(self.file_id).unwrap();
        if let Some(start) = file.source.find(target_pattern) {
            let end = start + target_pattern.len();

            // 1. Create your custom crate's LtxSpan tracking coordinate with the FileId
            let ltx_span = LtxSpan::new(start, end, self.file_id);

            // 2. Instantiate the core inner variant
            let inner_diagnostic: LtxDiagnosticInner = make_diagnostic(ltx_span).into();

            // 3. Create the LtxDiagnostic bundling the inner variant and source map
            let core_diagnostic = LtxDiagnostic::new(inner_diagnostic, self.source_map.clone());

            sink.push(core_diagnostic);
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
