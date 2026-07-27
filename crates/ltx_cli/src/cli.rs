//! CLI entry point and subcommand dispatch.

use crate::commands::code::CodeArgs;
use crate::commands::{CheckArgs, NewArgs, clean_build, run_code};
use crate::exit_code;
use clap::{Parser, Subcommand};
use ltx_config::{BibLayout, ScaffoldOptions, SrcLayout, scaffold};
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
    /// Check a LaTeX file for syntax errors.
    Check(CheckArgs),
    /// List all registered diagnostic error codes.
    Code(CodeArgs),
    /// clean a generated build files of latex
    Clean,
}

impl Ltx {
    /// Parses the CLI arguments and runs the corresponding subcommand.
    ///
    /// # Errors
    ///
    /// Returns a [`miette::Report`] if the subcommand fails.
    pub fn run(&self) -> miette::Result<()> {
        match &self.command {
            Command::New(args) => {
                let project_dir = PathBuf::from(args.name());
                let opts = ScaffoldOptions {
                    name: args.name().to_owned(),
                    engine: args.engine(),
                    src: if args.src() {
                        SrcLayout::WithSrcDir
                    } else {
                        SrcLayout::Flat
                    },
                    bib: if args.bib() {
                        BibLayout::WithBibDir
                    } else {
                        BibLayout::Flat
                    },
                };

                scaffold(&project_dir, &opts)?;

                eprintln!("Created project `{}`", args.name());
                Ok(())
            }
            Command::Check(args) => {
                let exit = crate::commands::check::run_check(args);
                if exit == exit_code::SUCCESS {
                    Ok(())
                } else {
                    std::process::exit(i32::from(exit));
                }
            }
            Command::Code(args) => {
                run_code(args);
                Ok(())
            }
            Command::Clean => {
                clean_build().unwrap();
                Ok(())
            }
        }
    }
}
