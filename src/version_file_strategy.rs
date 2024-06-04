use clap::ValueEnum;
use std::{default, fmt::Display};

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum VersionFileStrategy {
  #[default]
  Local,
  Recursive,
}

