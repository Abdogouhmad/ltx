use crate::ctx::{AppContext, CliCommand};
use clap::Args;
use ltx_diagnostics::ALL_CODES;

/// Arguments for `ltx code`.
///
/// Lists all diagnostic codes the tool can produce, optionally
/// filtered by severity.
///
/// Examples:
///   ltx code          — show all codes
///   ltx code -e       — errors only
///   ltx code -w       — warnings only
///   ltx code -e -w    — both (same as no flags)
#[derive(Args, Debug, Clone)]
pub struct CodeArgs {
    /// Show only error codes (prefix `LTX::E`).
    #[arg(short = 'e', long = "errors")]
    pub errors: bool,

    /// Show only warning codes (prefix `LTX::W`).
    #[arg(short = 'w', long = "warnings")]
    pub warnings: bool,
}

impl CliCommand for CodeArgs {
    fn execute(&self, _ctx: &AppContext) -> miette::Result<()> {
        let show_errors = !self.warnings || self.errors;
        let show_warnings = !self.errors || self.warnings;

        eprintln!("{:<14} {:<42} SEVERITY", "CODE", "DESCRIPTION");
        eprintln!("{}", "-".repeat(70));

        let mut count = 0usize;
        for entry in ALL_CODES {
            let is_error = entry.severity == "error";
            let is_warning = entry.severity == "warning";

            if (is_error && show_errors) || (is_warning && show_warnings) {
                eprintln!(
                    "{:<14} {:<42} {}",
                    entry.code, entry.description, entry.severity
                );
                count += 1;
            }
        }

        eprintln!("\n{count} total codes");
        Ok(())
    }
}
