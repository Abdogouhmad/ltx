use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_compiler::ALL_CODES as COMPILER_CODES;
use ltx_config::ALL_CODES as CONFIG_CODES;
use ltx_diagnostics::ErrorCode;
use ltx_lexer::ALL_CODES as LEXER_CODES;
use ltx_parser::ALL_CODES as PARSER_CODES;

/// Arguments for `ltx code`.
///
/// Lists all diagnostic codes the tool can produce, filtered by severity
/// (`-e` / `-w`) and/or by the phase that owns them
/// (`--lexer` / `--parser` / `--config` / `--compiler`).
///
/// Examples:
///   ltx code                 — show all codes (every phase)
///   ltx code -e              — errors only
///   ltx code -w              — warnings only
///   ltx code --lexer         — lexer codes only
///   ltx code --parser --compiler — parser and compiler codes
///   ltx code -e --lexer      — lexer errors only
#[derive(Args, Debug, Clone)]
pub struct CodeArgs {
    /// Show only error codes (prefix `LTX::*::E*`).
    #[arg(short = 'e', long = "errors")]
    pub errors: bool,

    /// Show only warning codes (prefix `LTX::*::W*`).
    #[arg(short = 'w', long = "warnings")]
    pub warnings: bool,

    /// Show only lexer codes (`LTX::LEXER::*`).
    #[arg(long = "lexer")]
    pub lexer: bool,

    /// Show only parser codes (`LTX::PARSER::*`).
    #[arg(long = "parser")]
    pub parser: bool,

    /// Show only config codes (`LTX::CONFIG::*`).
    #[arg(long = "config")]
    pub config: bool,

    /// Show only compiler codes (`LTX::COMPILER::*`).
    #[arg(long = "compiler")]
    pub compiler: bool,

    /// Show every phase (explicitly selects all phases, the default).
    #[arg(long = "all")]
    pub all: bool,
}

impl CodeArgs {
    /// The aggregated code registry across every crate.
    fn all_codes(&self) -> Vec<&'static ErrorCode> {
        let mut codes = Vec::new();
        let show = |flag: bool| {
            flag || self.all || !(self.lexer || self.parser || self.config || self.compiler)
        };
        if show(self.lexer) {
            codes.extend(LEXER_CODES);
        }
        if show(self.parser) {
            codes.extend(PARSER_CODES);
        }
        if show(self.config) {
            codes.extend(CONFIG_CODES);
        }
        if show(self.compiler) {
            codes.extend(COMPILER_CODES);
        }
        codes
    }
}

impl CliCommand for CodeArgs {
    fn execute(&self, _ctx: &AppContext) -> miette::Result<()> {
        let show_errors = !self.warnings || self.errors;
        let show_warnings = !self.errors || self.warnings;

        eprintln!("{:<24} {:<42} SEVERITY  PHASE", "CODE", "DESCRIPTION");
        eprintln!("{}", "-".repeat(80));

        let mut count = 0usize;
        for entry in self.all_codes() {
            let is_error = entry.severity == "error";
            let is_warning = entry.severity == "warning";

            if (is_error && show_errors) || (is_warning && show_warnings) {
                eprintln!(
                    "{:<24} {:<42} {:<9} {}",
                    entry.code, entry.description, entry.severity, entry.phase
                );
                count += 1;
            }
        }

        eprintln!("\n{count} total codes");
        Ok(())
    }
}
