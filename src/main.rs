use clap::Parser;
use std::path::Path;
use std::process::Command;
use log::info;

mod preprocessor;
mod compiler;
mod assembler;

#[derive(Parser)]
#[allow(non_snake_case)]
pub struct Cli {
  #[arg(long)]
  pub lex: bool,
  #[arg(long)]
  pub parse: bool,
  #[arg(long)]
  pub codegen: bool,
  #[arg(short = 'S')]
  pub S: bool,
  pub file_name: Option<String>,
}

impl Cli {
  fn check(&self) {
    let is_existed = Path::new(self.file_name.as_ref().expect("should give a file name")).exists();
    if !is_existed {
      panic!("File not found.");
    }
  }
}

fn main() {
  env_logger::init();

  let cli = Cli::parse();
  cli.check();

  preprocessor::run(&cli);
  compiler::run(&cli);
  assembler::run(&cli);
}
