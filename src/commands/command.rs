use crate::config::{self, FnmConfig};
use crate::outln;
use colored::Colorize;

pub trait Command: Sized {
    type Error: std::error::Error;

    fn apply(self, config: &FnmConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &FnmConfig) {
        let err_s = format!("{err}");
        outln!(config, Error, "{} {}", "error:".red().bold(), err_s.red());
        std::process::exit(1);
    }
}
