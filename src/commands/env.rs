use super::command::Command;
use crate::config::FnmConfig;
use crate::directories;
use crate::fs::symlink_dir;
use crate::outln;
use crate::path_ext::PathExt;
use crate::shell::{infer_shell, Shell, Shells};
use clap::ValueEnum;
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

#[derive(clap::Parser, Debug, Default)]
pub struct Env {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    shell: Option<Shells>,
    /// Print JSON instead of shell commands.
    #[clap(long, conflicts_with = "shell")]
    json: bool,
    /// Deprecated. This is the default now.
    #[clap(long, hide = true)]
    multi: bool,
    /// Print the script to change Node versions every directory change
    #[clap(long)]
    use_on_cd: bool,
}

fn generate_symlink_path() -> String {
    format!(
        "{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis(),
    )
}
