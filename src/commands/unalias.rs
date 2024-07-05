use super::command::Command;
use crate::fs::remove_symlink_dir;
use crate::user_version::UserVersion;
use crate::version::Version;
use crate::{choose_version_for_user_input, config::FnmConfig};
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Unalias {
    pub(crate) requested_alias: String,
}
