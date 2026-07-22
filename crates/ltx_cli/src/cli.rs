// src/cli.rs
use crate::commands::NewArgs;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ltx", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Ltx {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new ltx project
    New(NewArgs),
}

impl Ltx {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Command::New(args) => {
                println!("Creating new project: {}", args.name());
                if let Some(engine) = args.engine() {
                    println!("Using engine: {}", engine);
                }
                // Call your actual new-project logic here
                Ok(())
            }
        }
    }
}
