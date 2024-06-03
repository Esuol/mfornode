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

    pub fn writer_for(&self, logging: &Self) -> Box<dyn std::io::Write> {
        if self.is_writable(logging) {
            match logging {
                Self::Error => Box::from(std::io::stderr()),
                _ => Box::from(std::io::stdout()),
            }
        } else {
            Box::from(std::io::sink())
        }
    }

    pub fn possible_values() -> &'static [&'static str; 4] {
        &["quiet", "info", "all", "error"]
    }
}

impl From<LogLevel> for &'static str {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Quiet => "quiet",
            LogLevel::Error => "error",
            LogLevel::Info => "info",
        }
    }
}
