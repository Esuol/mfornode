use super::command::Command;
use crate::config::FnmConfig;
use crate::shell::{infer_shell, Shell};
use crate::{cli::Cli, shell::Shells};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{Generator, Shell as ClapShell};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[clap(long)]
    shell: Option<Shells>,
}


impl Command for Completions {
  type Error = Error;

  fn apply(self, _config: &FnmConfig) -> Result<(), Self::Error> {
      let mut stdio = std::io::stdout();
      let shell: Box<dyn Shell> = self
          .shell
          .map(Into::into)
          .or_else(|| infer_shell().map(Into::into))
          .ok_or(Error::CantInferShell)?;
      let shell: ClapShell = shell.into();
      let mut app = Cli::command();
      app.build();
      shell.generate(&app, &mut stdio);
      Ok(())
  }
}