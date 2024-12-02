mod scanner;
pub fn run(source: &str) -> Vec<scanner::TOKEN> {
	let mut scanner = scanner::Scanner::new(source);
	scanner.scan_token().expect("Failed to scan token")
}
