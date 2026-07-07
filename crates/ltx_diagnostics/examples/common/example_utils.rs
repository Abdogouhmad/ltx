use ltx_diagnostics::{
    LtxDiagnostic, LtxDiagnosticInner, LtxDiagnosticSink, LtxFileId, LtxSourceMap, LtxSpan,
};
use std::{error::Error, sync::Arc};

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
    pub fn trigger_and_render<F, D>(
        &self,
        target_pattern: &str,
        make_diagnostic: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(LtxSpan) -> D,
        D: Into<LtxDiagnosticInner>,
    {
        let mut sink = LtxDiagnosticSink::new();

        let file = self
            .source_map
            .get_file(self.file_id)
            .ok_or("File not found in source map")?;

        if let Some(start) = file.source.find(target_pattern) {
            let end = start + target_pattern.len();

            let ltx_span = LtxSpan::new(start, end, self.file_id);
            let inner_diagnostic: LtxDiagnosticInner = make_diagnostic(ltx_span).into();
            let core_diagnostic = LtxDiagnostic::new(inner_diagnostic, self.source_map.clone());

            sink.push(core_diagnostic);
        } else {
            panic!(
                "Test setup error: Could not find substring '{target_pattern}' in the source mock string."
            );
        }

        println!("🔍 Found {} issue(s):\n", sink.len());
        print!("{}", sink.render_pretty());
        Ok(())
    }
}
