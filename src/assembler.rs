use crate::Cli;
use log::info;
use std::process::Command;

pub fn run(cli: &Cli) {
	let raw_name = cli.file_name.as_ref().unwrap();
	let src_name = raw_name.replace(".c", ".s");
	let target_name = raw_name.replace(".c", "");
	info!("Assembling {} to {}", src_name, target_name);

	Command::new("clang")
		.args(["-o", &target_name, &src_name])
		.status()
		.expect("Failed to run assembler.");
}
