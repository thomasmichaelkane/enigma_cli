use crate::cli::Cli;

pub struct Config {
  debug_mode: bool,
  secret_mode: bool,
  show_instructions: bool,
  animate: bool,
}

impl Config {
  pub fn new(cli: &Cli) -> Self {
    Self {
      debug_mode: cli.debug_mode,
      secret_mode: cli.secret_mode,
      show_instructions: cli.show_instructions,
      animate: cli.animate,
    }
  }

  pub fn is_debug(&self) -> bool {
    self.debug_mode
  }

  pub fn is_display(&self) -> bool {
    !self.debug_mode
  }

  pub fn is_secret(&self) -> bool {
    self.secret_mode
  }

  pub fn should_show_instructions(&self) -> bool {
    self.show_instructions
  }

  pub fn animate(&self) -> bool {
    self.animate
  }
  
}