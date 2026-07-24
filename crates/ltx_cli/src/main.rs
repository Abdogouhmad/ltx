mod cli;
mod commands;
mod error;
mod exit_code;

use clap::Parser;
use cli::Ltx;
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
