//! `ltx code` command — list diagnostic error codes.

use clap::Args;
use ltx_diagnostics::ALL_CODES;

/// Arguments for the `code` subcommand.
#[derive(Args, Debug, Clone)]
pub struct CodeArgs {
    /// Show only error codes (LTX::E***).
    #[arg(short = 'e', long = "errors")]
    pub errors: bool,

    /// Show only warning codes (LTX::W***).
    #[arg(short = 'w', long = "warnings")]
    pub warnings: bool,
}

/// Print diagnostic codes filtered by the given flags.
///
/// - No flags: print all codes.
/// - `-e`: errors only.
/// - `-w`: warnings only.
/// - `-e -w`: both (same as no flags).
pub fn run_code(args: &CodeArgs) {
    let show_errors = !args.warnings || args.errors;
    let show_warnings = !args.errors || args.warnings;

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
}
