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
