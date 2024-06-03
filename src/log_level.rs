use std::{default, fmt::Display};

use clap::ValueEnum;

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, ValueEnum)]
pub enum LogLevel {
    Quiet,
    Error,
    #[default]
    #[value(alias("all"))]
    Info,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Quiet => write!(f, "quiet"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Info => write!(f, "info"),
        }
    }
}

impl LogLevel {
    pub fn is_writable(&self, logging: &Self) -> bool {
        use std::cmp::Ordering;
        matches!(self.cmp(logging), Ordering::Greater | Ordering::Equal)
    }
}
