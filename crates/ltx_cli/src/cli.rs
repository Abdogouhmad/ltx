//! CLI entry point and subcommand dispatch.

use crate::commands::NewArgs;
use clap::{Parser, Subcommand};
use ltx_config::{ScaffoldOptions, scaffold};
use owo_colors::OwoColorize;
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
}

impl Ltx {
    /// Parses the CLI arguments and runs the corresponding subcommand.
    ///
    /// # Errors
    ///
    /// Returns an error if the subcommand fails (e.g. I/O during scaffolding).
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Command::New(args) => {
                let project_dir = PathBuf::from(args.name());
                let opts = ScaffoldOptions {
                    name: args.name().to_owned(),
                    engine: args.engine().unwrap_or("pdflatex").to_owned(),
                    src: args.src(),
                    bib: args.bib(),
                };

                scaffold(&project_dir, &opts)?;

                // priny in green color
                println!("Created project `{}`", args.name().green());
                Ok(())
            }
        }
    }
}
