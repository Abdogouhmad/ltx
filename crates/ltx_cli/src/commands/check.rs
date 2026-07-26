//! `check` command — lex, parse, and diagnostics for a `.tex` file.

use clap::Args;
use ltx_diagnostics::LtxSourceMap;
use ltx_lexer::{LtxLexer, TokenStream};
use ltx_parser::parse_document;
use std::path::PathBuf;
use std::sync::Arc;

use crate::exit_code;

#[derive(Args, Debug, Clone)]
pub struct CheckArgs {
    /// A path to a .tex file to check.
    pub path: Option<PathBuf>,
}

impl CheckArgs {
    /// Returns the file path, if provided.
    #[must_use]
    pub fn path_to_afile(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}

/// Runs the full lex → parse → diagnostics pipeline on a `.tex` file.
///
/// Returns a process exit code: 0 for clean, non-zero for errors.
pub fn run_check(args: &CheckArgs) -> u8 {
    let path = match args.path_to_afile() {
        Some(p) => p.clone(),
        None => {
            eprintln!("No file path provided.");
            return exit_code::FILE_NOT_FOUND;
        }
    };

    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading `{}`: {e}", path.display());
            return exit_code::FILE_NOT_FOUND;
        }
    };

    let mut source_map = LtxSourceMap::new();
    let file_id = source_map.add_file(path.clone()).unwrap_or_else(|e| {
        eprintln!("Error loading `{}`: {e}", path.display());
        std::process::exit(i32::from(exit_code::ERROR));
    });

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
        exit_code::DIAGNOSTICS_FOUND
    } else if total > 0 {
        eprintln!("Check passed with {total} warning(s).");
        exit_code::SUCCESS
    } else {
        eprintln!("Check passed — no issues found.");
        exit_code::SUCCESS
    }
}
