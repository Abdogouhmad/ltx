use crate::ctx::{AppContext, CliCommand};
use crate::error::CliError;
use clap::Args;
use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::parse_document;
use std::path::PathBuf;
use std::sync::Arc;

/// Arguments for `ltx check`.
///
/// Runs the full lex → parse → diagnostics pipeline on a `.tex` file.
/// Reports syntax errors and warnings without producing any output files.
///
/// Exit codes:
///   0 — no issues found (or only warnings)
///   4 — one or more errors found
#[derive(Args, Debug, Clone)]
pub struct CheckArgs {
    /// Path to a `.tex` file to lint and parse.
    ///
    /// If omitted, an error is shown.
    pub path: Option<PathBuf>,
}

impl CliCommand for CheckArgs {
    fn execute(&self, _ctx: &AppContext) -> miette::Result<()> {
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| miette::miette!("No file path provided."))?;

        let source = std::fs::read_to_string(path)
            .map_err(|e| miette::miette!("Error reading `{}`: {e}", path.display()))?;

        let mut source_map = LtxSourceMap::new();
        let file_id = source_map
            .add_file(path.clone())
            .map_err(|e| miette::miette!("Error loading `{}`: {e}", path.display()))?;

        let source_map = Arc::new(source_map);
        let stream = TokenStream::new(LtxLexer::new(&source, file_id, source_map.as_ref().clone()));

        let mut parser = ltx_parser::LtxParser::new(stream);
        let _doc = parse_document(&mut parser);

        let handler = parser.error_handler_mut();
        let has_errors = handler.has_errors();
        let total = handler.total_count();

        if total > 0 {
            eprintln!("{}", handler.render_pretty());
        }

        if has_errors {
            eprintln!("Check failed with {} error(s).", handler.error_count());
            Err(CliError::DiagnosticsFound.into())
        } else if total > 0 {
            eprintln!("Check passed with {total} warning(s).");
            Ok(())
        } else {
            eprintln!("Check passed — no issues found.");
            Ok(())
        }
    }
}
