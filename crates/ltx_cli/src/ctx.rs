use std::{path::PathBuf, str::FromStr};

/// Controls the format of status messages printed by the tool.
#[derive(Debug, Clone, Copy, Default)]
pub enum OutputFormat {
    /// Human-readable terminal output with colour and formatting.
    #[default]
    Human,
    /// Machine-readable JSON output for editor/IDE integration.
    Json,
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            _ => Err("expected 'human' or 'json'"),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "human"),
            Self::Json => write!(f, "json"),
        }
    }
}

/// Shared context passed to every command.
pub struct AppContext {
    pub manifest_path: Option<PathBuf>,
    pub format: OutputFormat,
    pub verbose: u8,
}

/// Trait implemented by all CLI subcommand argument types.
pub trait CliCommand {
    fn execute(&self, ctx: &AppContext) -> miette::Result<()>;
}
