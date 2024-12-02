use crate::Cli;
use std::fs;
use log::debug;

mod lexer;
mod parser;
mod codegen;
mod emit;
mod common;

pub fn run(cli: &Cli) {
	let raw_name = cli.file_name.as_ref().expect("should give a file name");
	let src_name = raw_name.replace(".c", ".i");
	let source = fs::read_to_string(&src_name).expect("Failed to read file");
	debug!("src_name: {}", src_name);
	match cli {
		Cli { lex: true, .. } => run_until_lexer(&source),
		Cli { parse: true, .. } => run_until_parser(&source),
		Cli { codegen: true, .. } => run_until_codegen(&source),
		Cli { S: true, .. } => run_until_emit(&source),
		_ => panic!("Unexpected config."),
	};
}

fn run_until_lexer(source: &str) {
	let tokens = lexer::run(source);
}

fn run_until_parser(source: &str) {
	let tokens = lexer::run(source);
	parser::run();
}

fn run_until_codegen(source: &str) {
	let tokens = lexer::run(source);
	parser::run();
	codegen::run();
}

fn run_until_emit(source: &str) {
	let tokens = lexer::run(source);
	parser::run();
	codegen::run();
	emit::run();
}
