use super::command::Command;
use crate::config::FnmConfig;
use crate::shell::{infer_shell, Shell};
use crate::{cli::Cli, shell::Shells};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{Generator, Shell as ClapShell};
use thiserror::Error;
