use super::command::Command as Cmd;
use crate::choose_version_for_user_input::{
    choose_version_for_user_input, Error as UserInputError,
};
use crate::config::FnmConfig;
use crate::outln;
use crate::user_version::UserVersion;
use crate::user_version_reader::UserVersionReader;
use colored::Colorize;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, clap::Parser)]
#[clap(trailing_var_arg = true)]
pub struct Exec {
    /// Either an explicit version, or a filename with the version written in it
    #[clap(long = "using")]
    version: Option<UserVersionReader>,
    /// Deprecated. This is the default now.
    #[clap(long = "using-file", hide = true)]
    using_file: bool,
    /// The command to run
    arguments: Vec<String>,
}

impl Exec {
  pub(crate) fn new_for_version(
      version: &crate::version::Version,
      cmd: &str,
      arguments: &[&str],
  ) -> Self {
      let reader = UserVersionReader::Direct(UserVersion::Full(version.clone()));
      let args: Vec<_> = std::iter::once(cmd)
          .chain(arguments.iter().copied())
          .map(String::from)
          .collect();
      Self {
          version: Some(reader),
          using_file: false,
          arguments: args,
      }
  }
}