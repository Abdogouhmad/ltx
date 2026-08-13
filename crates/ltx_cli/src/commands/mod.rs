pub mod build;
pub mod check;
pub mod clean;
pub mod code;
pub mod new;
#[cfg(feature = "self-update")]
pub mod update;
pub mod watch;

pub use build::BuildArgs;
pub use check::CheckArgs;
pub use clean::CleanArgs;
pub use code::CodeArgs;
pub use new::NewArgs;
#[cfg(feature = "self-update")]
pub use update::UpdateArgs;
pub use watch::WatchArgs;

use std::path::{Path, PathBuf};

/// Resolves the manifest file, defaulting to `ltx.toml` in the current
/// directory when `--manifest-path` is not given.
pub fn resolve_manifest_path(manifest_path: Option<&Path>) -> miette::Result<PathBuf> {
    let path = manifest_path.unwrap_or_else(|| Path::new("ltx.toml"));
    if !path.is_file() {
        return Err(miette::miette!(
            "no ltx.toml found at `{}` — run `ltx new` to scaffold a project",
            path.display()
        ));
    }
    Ok(path.to_path_buf())
}
