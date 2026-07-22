// src/main.rs
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
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::FAILURE
        }
    }
}
