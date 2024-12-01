use crate::Cli;
use std::process::Command;
use log::info;

pub fn run(cli: &Cli) {
	let raw_name = cli.file_name.as_ref().unwrap();
	let src_name = raw_name;
	let target_name = raw_name.replace(".c", ".ii");
	info!("Preprocessing {} to {}", src_name, target_name);

	

	Command::new("clang")
		.args(["-E", "-P", "-o", &target_name, src_name])
		.status()
		.expect("Failed to run assembler.");
}
