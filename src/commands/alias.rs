use super::command::Command;
use crate::alias::create_alias;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as ApplicableVersionError,
};
use crate::config::FnmConfig;
use crate::user_version::UserVersion;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
pub struct Alias {
    pub(crate) to_version: UserVersion,
    pub(crate) name: String,
}
