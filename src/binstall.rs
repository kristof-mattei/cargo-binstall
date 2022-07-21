use std::path::PathBuf;

use crate::PkgOverride;

mod resolve;
pub use resolve::*;

mod install;
pub use install::*;

pub struct Options {
    pub no_symlinks: bool,
    pub dry_run: bool,
    pub version: Option<String>,
    pub manifest_path: Option<PathBuf>,
    pub cli_overrides: PkgOverride,
}
