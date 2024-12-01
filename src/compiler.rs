use crate::Cli;

mod lexer;
mod parser;
mod codegen;
mod emit;
mod common;

pub fn run(cli: &Cli) {
	match cli {
		Cli { lex: true, .. } => run_until_lexer(),
		Cli { parse: true, .. } => run_until_parser(),
		Cli { codegen: true, .. } => run_until_codegen(),
		Cli { S: true, .. } => run_until_emit(),
		_ => panic!("Unexpected config."),
	};
}

fn run_until_lexer() {
	lexer::run();
}

fn run_until_parser() {
	lexer::run();
	parser::run();
}

fn run_until_codegen() {
	lexer::run();
	parser::run();
	codegen::run();
}

fn run_until_emit() {
	lexer::run();
	parser::run();
	codegen::run();
	emit::run();
}
