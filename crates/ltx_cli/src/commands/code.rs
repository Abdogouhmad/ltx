use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_compiler::ALL_CODES as COMPILER_CODES;
use ltx_config::ALL_CODES as CONFIG_CODES;
use ltx_diagnostics::ErrorCode;
use ltx_lexer::ALL_CODES as LEXER_CODES;
use ltx_linter::ALL_CODES as LINTER_CODES;
use ltx_parser::ALL_CODES as PARSER_CODES;

/// Arguments for `ltx code`.
///
/// Lists all diagnostic codes the tool can produce, filtered by severity
/// (`-e` / `-w`) and/or by the phase that owns them
/// (`--lexer` / `--parser` / `--config` / `--compiler` / `--lint`).
///
/// The linter owns the global registry: by default only the unified
/// `LTX::LINTER::*` codes are shown. The lexer, parser, config and compiler
/// registries are internal — they stay available for rare cross-crate use and
/// are listed only when their phase flag or `--all` is given.
///
/// Examples:
///   ltx code                 — global linter codes (default)
///   ltx code -e              — global errors only
///   ltx code -w              — global warnings only
///   ltx code --all           — every code across every crate
///   ltx code --lexer         — internal lexer codes
///   ltx code --parser --compiler — parser and compiler codes
///   ltx code --lint          — unified linter codes (same as default)
///   ltx code -e --lexer      — internal lexer errors only
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

    /// Show the global linter codes (`LTX::LINTER::*`).
    #[arg(long = "lint")]
    pub lint: bool,

    /// Show every phase, including the internal code registries.
    #[arg(long = "all")]
    pub all: bool,
}

impl CodeArgs {
    /// The aggregated code registry across every crate.
    ///
    /// The linter owns the GLOBAL registry — the unified `LTX::LINTER::`
    /// codes a user sees. The lexer, parser, config and compiler registries
    /// are internal: they are listed only when their phase flag (or `--all`)
    /// is given, since their codes appear globally under the linter namespace
    /// anyway.
    fn all_codes(&self) -> Vec<ErrorCode> {
        let mut codes = Vec::new();
        let any_phase = self.lexer || self.parser || self.config || self.compiler || self.lint;

        if self.lexer {
            codes.extend(LEXER_CODES.iter().copied());
        }
        if self.parser {
            codes.extend(PARSER_CODES.iter().copied());
        }
        if self.config {
            codes.extend(CONFIG_CODES.iter().copied());
        }
        if self.compiler {
            codes.extend(COMPILER_CODES.iter().copied());
        }
        if self.lint || self.all {
            codes.extend(LINTER_CODES.iter().copied());
        }
        if !any_phase {
            // No phase filter: show the global registry (linter) only.
            codes.extend(LINTER_CODES.iter().copied());
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
        for entry in &self.all_codes() {
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
