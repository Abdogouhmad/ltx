// TODO: an impl for checking command using `ltx_parser`
// NOTE(lib): - use utils -> will use `ltx_diagnostics`
//            - parse functions

use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct CheckArgs {
    /// a path to a .tex file to check
    pub path: Option<PathBuf>,
}

impl CheckArgs {
    // a convenience method to get the path as a `&PathBuf`
    pub fn path_to_afile(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
