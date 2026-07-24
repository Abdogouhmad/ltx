//! CLI entry point and subcommand dispatch.

use crate::commands::NewArgs;
use clap::{Parser, Subcommand};
use ltx_config::{ScaffoldOptions, scaffold};
use std::path::PathBuf;

/// Top-level CLI parser for the `ltx` binary.
#[derive(Debug, Parser)]
#[command(name = "ltx", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Ltx {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new ltx project with starter files.
    New(NewArgs),
    /// Check a latex file from any syntax errors
    Check,
    /// List all registered diagnostic error codes.
    Code,
}

impl Ltx {
    /// Parses the CLI arguments and runs the corresponding subcommand.
    ///
    /// # Errors
    ///
    /// Returns a [`miette::Report`] if the subcommand fails.
    pub fn run(&self) -> miette::Result<()> {
        match &self.command {
            // new command
            Command::New(args) => {
                let project_dir = PathBuf::from(args.name());
                let opts = ScaffoldOptions {
                    name: args.name().to_owned(),
                    engine: args.engine(),
                    src: args.src(),
                    bib: args.bib(),
                };

                scaffold(&project_dir, &opts)?;

                println!("Created project `{}`", args.name());
                Ok(())
            }
            // TODO: check command with file path I will use parser
            Command::Check => {
                println!("checking");
                Ok(())
            }
            Command::Code => {
                use ltx_diagnostics::ALL_CODES;

                println!("{:<14} {:<42} SEVERITY", "CODE", "DESCRIPTION");
                println!("{}", "-".repeat(70));
                for entry in ALL_CODES {
                    println!(
                        "{:<14} {:<42} {}",
                        entry.code, entry.description, entry.severity
                    );
                }
                println!("\n{} total codes", ALL_CODES.len());
                Ok(())
            }
        }
    }
}
