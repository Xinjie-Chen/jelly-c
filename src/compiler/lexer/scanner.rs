use std::fs;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum TOKEN {
	INT,
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	IDENTIFIER,
	Const,
}

struct Scanner {
	source: Vec<char>,
	start: i32,
	current: i32,
}

impl Scanner {
	fn new(file_name: &str) -> Scanner {
		Scanner {
			source: fs::read_to_string(file_name).unwrap().chars().collect(),
			start: 0,
			current: 0,
		}
	}

	fn scan_token(&mut self) -> Vec<TOKEN> {
		let mut tokens: Vec<TOKEN> = Vec::new();
		while self.current < self.source.len() as i32 {
			let c = self.advance();
			match c {
				'(' => tokens.push(TOKEN::LeftParen),
				')' => tokens.push(TOKEN::RightParen),
				'{' => tokens.push(TOKEN::LeftBrace),
				'}' => tokens.push(TOKEN::RightBrace),
				_ => tokens.push(TOKEN::IDENTIFIER),
			}
		}
		tokens
	}

	fn advance(&mut self) -> char{
		let result = self.at();
		self.current += 1;
		result
	}

	fn at(&self) -> char {
		self.source[self.current as usize]
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let scanner = Scanner::new("test.txt");
		assert_eq!(scanner.source, vec!['{', '}']);
		assert_eq!(scanner.start, 0);
		assert_eq!(scanner.current, 0);
	}

	#[test]
	fn test_scan_token() {
		let mut scanner = Scanner::new("test.txt");
		let tokens = scanner.scan_token();
		assert_eq!(tokens, vec![TOKEN::LeftBrace, TOKEN::RightBrace]);
	}
}