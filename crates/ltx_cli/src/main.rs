use clap::Parser;
use ltx_cli::cli::Ltx;
use std::process::ExitCode;

fn main() -> ExitCode {
    match Ltx::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(report) => {
            eprint!("{report:?}");
            ExitCode::FAILURE
        }
    }
}
