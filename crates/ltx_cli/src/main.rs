use clap::Parser;
use ltx_cli::cli::Cli;
use ltx_cli::error::CliError;
use std::process::ExitCode;

fn main() -> ExitCode {
    match Cli::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(report) => {
            eprint!("{report:?}");
            if let Some(CliError::DiagnosticsFound) = report.downcast_ref::<CliError>() {
                ExitCode::from(4)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
