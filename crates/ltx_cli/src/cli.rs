use crate::commands::build::BuildArgs;
use crate::commands::check::CheckArgs;
use crate::commands::clean::CleanArgs;
use crate::commands::code::CodeArgs;
use crate::commands::new::NewArgs;
#[cfg(feature = "self-update")]
use crate::commands::update::UpdateArgs;
use crate::commands::watch::WatchArgs;
use crate::ctx::{AppContext, CliCommand, OutputFormat};
#[cfg(feature = "self-update")]
use crate::update_check;
use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

/// A fast, opinionated LaTeX toolchain — lint, format, compile.
#[derive(Debug, Parser)]
#[command(name = "ltx", version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to a manifest file (`ltx.toml`) for project-level configuration.
    ///
    /// When set, file paths are resolved relative to the manifest's directory.
    #[arg(long, global = true)]
    pub manifest_path: Option<PathBuf>,

    /// Output format for status messages.
    ///
    /// `human` — readable messages intended for terminal display (default).
    /// `json`   — machine-readable JSON output, useful for editor/IDE integration.
    #[arg(long, global = true, default_value_t = OutputFormat::Human)]
    pub message_format: OutputFormat,

    /// Verbosity level.
    ///
    /// Repeat to increase: `-v`, `-vv`, `-vvv`.
    #[arg(short, long, global = true, action = ArgAction::Count)]
    pub verbose: u8,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new LaTeX project with starter files and sensible defaults.
    ///
    /// Generates a project directory containing a `main.tex`, optional
    /// `src/` / `bib/` layout, and a `ltx.toml` manifest.
    New(NewArgs),

    /// Lex, parse, and run diagnostics on `.tex` files without producing output.
    ///
    /// Checks every `.tex` file in the project recursively (like `cargo check`);
    /// pass `-p <file.tex>` to check a single file. Exits with code 4 when
    /// diagnostics are found, making it suitable for CI pipelines.
    Check(CheckArgs),

    /// List all registered diagnostic error/warning codes.
    ///
    /// Each code (e.g. `LTX::E001`) has a severity and description. Use
    /// `-e` / `-w` to filter by severity.
    Code(CodeArgs),

    /// Remove generated build artifacts from the `target/` directory.
    ///
    /// Prints a summary of removed files and total size, similar to
    /// `cargo clean`.
    Clean(CleanArgs),

    /// Compile the project into a PDF using the engine from `ltx.toml`.
    ///
    /// The input file comes from `[project].main` and the output PDF name
    /// from `[build].name`. Only the `tectonic` engine is currently wired up.
    Build(BuildArgs),

    /// Watch the project for changes and rebuild the PDF on save.
    ///
    /// Compiles once on startup, then recompiles whenever a relevant file
    /// (`.tex`, `.sty`, `.cls`, `.bib`) changes. Output churn under `target/`
    /// is ignored. Use `--manifest-path` to point at a specific `ltx.toml`.
    Watch(WatchArgs),

    /// Update ltx to the latest GitHub release.
    ///
    /// Downloads the release for the current platform and replaces the running
    /// binary. Use `--check` to inspect without installing, or `--yes` to skip
    /// the confirmation prompt. Set `LTX_NO_UPDATE_CHECK` to silence the
    /// background update hint printed by other commands.
    #[cfg(feature = "self-update")]
    #[command(alias = "self-update")]
    Update(UpdateArgs),
}

impl Cli {
    pub fn run(&self) -> miette::Result<()> {
        #[cfg(feature = "self-update")]
        if !matches!(self.command, Command::Update(_)) {
            update_check::maybe_notify();
        }

        let ctx = AppContext {
            manifest_path: self.manifest_path.clone(),
            format: self.message_format,
            verbose: self.verbose,
        };
        match &self.command {
            Command::New(args) => args.execute(&ctx),
            Command::Check(args) => args.execute(&ctx),
            Command::Code(args) => args.execute(&ctx),
            Command::Clean(args) => args.execute(&ctx),
            Command::Build(args) => args.execute(&ctx),
            Command::Watch(args) => args.execute(&ctx),
            #[cfg(feature = "self-update")]
            Command::Update(args) => args.execute(&ctx),
        }
    }
}
