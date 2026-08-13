//! Best-effort, background update notifications.
//!
//! Fires a detached thread that checks the GitHub releases once per invocation
//! and prints a hint to stderr when a newer release exists. Disable with the
//! `LTX_NO_UPDATE_CHECK` environment variable.

use owo_colors::OwoColorize;
use self_update::backends::github::Update;
use self_update::version::bump_is_greater;

const REPO_OWNER: &str = "Abdogouhmad";
const REPO_NAME: &str = "ltx";

/// Spawn a background check for a new release and notify on stderr if one is
/// available. All failures are swallowed: this is purely informational.
pub fn maybe_notify() {
    if std::env::var_os("LTX_NO_UPDATE_CHECK").is_some() {
        return;
    }

    std::thread::spawn(|| {
        let Ok(update) = Update::configure()
            .repo_owner(REPO_OWNER)
            .repo_name(REPO_NAME)
            .bin_name("ltx")
            .current_version(env!("CARGO_PKG_VERSION"))
            .identifier(archive_identifier())
            .show_output(false)
            .no_confirm(true)
            .build()
        else {
            return;
        };

        let Ok(latest) = update.get_latest_release() else {
            return;
        };

        let current = env!("CARGO_PKG_VERSION");
        if bump_is_greater(current, &latest.version).unwrap_or(false) {
            let msg = format!(
                "a new release of ltx is available (v{}) — run `ltx update` to upgrade",
                latest.version
            );
            eprintln!("{}", msg.cyan().bold());
        }
    });
}

fn archive_identifier() -> &'static str {
    if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    }
}
