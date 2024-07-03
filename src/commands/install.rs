use super::command::Command;
use crate::alias::create_alias;
use crate::arch::get_safe_arch;
use crate::config::FnmConfig;
use crate::downloader::{install_node_dist, Error as DownloaderError};
use crate::lts::LtsType;
use crate::outln;
use crate::remote_node_index;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::version_files::get_user_version_for_directory;
use anyhow::Error;
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    /// A version string. Can be a partial semver or a LTS version name by the format lts/NAME
    pub version: Option<UserVersion>,

    /// Install latest LTS
    #[clap(long, conflicts_with_all = &["version", "latest"])]
    pub lts: bool,

    /// Install latest version
    #[clap(long, conflicts_with_all = &["version", "lts"])]
    pub latest: bool,
}

impl Install {
    fn version(self) -> Result<Option<UserVersion>, Error> {
        match self {
            Self {
                version: v,
                lts: false,
                latest: false,
            } => Ok(v),
            Self {
                version: None,
                lts: true,
                latest: false,
            } => Ok(Some(UserVersion::Full(Version::Lts(LtsType::Latest)))),
            Self {
                version: None,
                lts: false,
                latest: true,
            } => Ok(Some(UserVersion::Full(Version::Latest))),
            _ => Err(Error::TooManyVersionsProvided),
        }
    }
}

impl Command for Install {
  type Error = Error;
}
