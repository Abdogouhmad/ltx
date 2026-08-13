use crate::ctx::{AppContext, CliCommand};
use crate::error::CliError;
use clap::Args;
use owo_colors::OwoColorize;
use self_update::Status;
use self_update::backends::github::Update;
use self_update::update::ReleaseUpdate;
use self_update::version::bump_is_greater;

/// GitHub repository the release binaries are published to.
const REPO_OWNER: &str = "Abdogouhmad";
const REPO_NAME: &str = "ltx";

/// Arguments for `ltx update`.
///
/// Checks the GitHub releases for the current platform, downloads the newest
/// release and replaces the running binary. Requires the binary to have been
/// installed from a GitHub release (e.g. via the cargo-dist installers).
///
/// Exit codes:
///   0 — up to date, or updated successfully
///   1 — check or update failed
#[derive(Debug, Clone, Args)]
pub struct UpdateArgs {
    /// Check for a newer release and report the result without installing.
    #[arg(long)]
    pub check: bool,

    /// Skip the confirmation prompt and install the new release immediately.
    #[arg(short = 'y', long)]
    pub yes: bool,
}

impl CliCommand for UpdateArgs {
    fn execute(&self, _ctx: &AppContext) -> miette::Result<()> {
        let update = configure_update(self.yes).map_err(CliError::Update)?;
        let current = env!("CARGO_PKG_VERSION");

        if self.check {
            let latest = update.get_latest_release().map_err(CliError::Update)?;
            if bump_is_greater(current, &latest.version).map_err(CliError::Update)? {
                println!("{}", "A new release is available".green().bold());
                println!(
                    "  current:  v{current}\n  latest:   v{}\n\nRun `ltx update` to upgrade.",
                    latest.version
                );
            } else {
                println!("{}", "You are up to date".green().bold());
                println!("  latest release: v{}", latest.version);
            }
            return Ok(());
        }

        match update.update().map_err(CliError::Update)? {
            Status::Updated(latest) => {
                println!("{}", "Update complete".green().bold());
                println!("  ltx updated from v{current} to v{latest}");
            }
            Status::UpToDate(_) => {
                println!("{}", "Already up to date".cyan().bold());
                println!("  ltx is at the latest release (v{current})");
            }
        }

        Ok(())
    }
}

/// Build a GitHub `Update` configured for this binary's platform.
///
/// The release assets are named `ltx_cli-{target}.{ext}` (`.tar.gz` on Unix,
/// `.zip` on Windows). The archive for the current target is selected with the
/// platform-specific extension as an identifier so that the `-update`, `.msi`
/// and `.sha256` assets — which also embed the target triple — are skipped.
fn configure_update(
    no_confirm: bool,
) -> Result<Box<dyn ReleaseUpdate>, self_update::errors::Error> {
    let identifier = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };

    Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("ltx")
        .current_version(env!("CARGO_PKG_VERSION"))
        .identifier(identifier)
        .no_confirm(no_confirm)
        .build()
}
