use std::{clone, fs};
use log::debug;
use std::collections::HashMap;

macro_rules! hash {
	($($key_value:expr),+) => {{
		let mut h = HashMap::new();
		$(h.insert($key_value.0, $key_value.1);)+
		h
	}}
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum TOKEN {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Semicolon,
	Int,
	Return,
	Void,
	CONST(i32),
	IDENTIFIER(String),
}

pub struct Scanner<'a> {
	source: &'a str,
	start: i32,
	current: i32,
	line: i32,
	keywords: HashMap<String, TOKEN>,
	err: bool,
}

#[allow(dead_code)]
impl<'a> Scanner<'a> {
	pub fn new(source: &'a str) -> Self {
		Scanner {
			source,
			start: 0,
			current: 0,
			line: 1,
			keywords: hash![
				("int".to_string(), TOKEN::Int),
				("void".to_string(), TOKEN::Void),
				("return".to_string(), TOKEN::Return)
			],
			err: false,
		}
	}

	pub fn scan_token(&mut self) -> Result<Vec<TOKEN>, Vec<ScanErr>> {
		let mut tokens: Vec<TOKEN> = Vec::new();
		let mut errors: Vec<ScanErr> = Vec::new();
		while self.current < self.source.len() as i32 {
			self.start = self.current;
			let c = self.advance();
			match c {
				'(' => tokens.push(TOKEN::LeftParen),
				')' => tokens.push(TOKEN::RightParen),
				'{' => tokens.push(TOKEN::LeftBrace),
				'}' => tokens.push(TOKEN::RightBrace),
				';' => tokens.push(TOKEN::Semicolon),
				' ' => (),
				'\t' => (),
				'\n' => self.line += 1,
				_ => {
					if c.is_ascii_digit() {
						while self.peek().is_ascii_digit() {
							self.advance();
						}
						if self.peek().is_alphabetic() || self.peek() == '_' {
							self.err = true;
							errors.push(ScanErr {
								line: self.line,
								message: "Invalid number".to_string(),
							});
						}
						let num: i32 = self.source[self.start as usize..self.current as usize]
							.parse()
							.expect("Unable to parse");
						tokens.push(TOKEN::CONST(num));
					} else if c.is_alphabetic() || c == '_' {
						while self.peek().is_alphabetic() || self.peek() == '_' {
							self.advance();
						}
						let value = self.source[self.start as usize..self.current as usize].to_string();
						if self.keywords.contains_key(&value) {
							tokens.push(self.keywords.get(&value).unwrap().clone());
						} else {
							tokens.push(TOKEN::IDENTIFIER(value));
						}
					} else {
						self.err = true;
						errors.push(ScanErr {
							line: self.line,
							message: format!("Unexpected character: {}", c),
						});
					}
				} 
			}
		}
		match self.err {
			true => Err(errors),
			false => Ok(tokens)
		}
	}

	fn advance(&mut self) -> char{
		let result = self.at();
		self.current += 1;
		result
	}

	fn at(&self) -> char {
		self.source
			.chars()
			.nth(self.current as usize)
			.expect("Input must be valid ascii")
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.source.len() as i32
	}

	fn matches(&mut self, expected: char) -> bool {
		if self.is_at_end() {
			return false
		}
		if self.at() != expected {
			return false
		}
		self.current += 1;
		true
	}

	fn peek(&self) -> char {
		if self.is_at_end() {
			return '\0'
		}
		self.at()
	}

	fn match_next(&mut self, expected: char, valid: TOKEN, invalid: TOKEN) -> TOKEN {
		if self.matches(expected) {
			return valid
		}
		invalid
	}
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct ScanErr {
	line: i32,
	message: String,
}


#[cfg(test)]
mod tests {
	use super::*;
	use rand::Rng;

	#[test]
	fn test_scan_token_valid() {
		env_logger::init();
		let rand_num = rand::thread_rng().gen_range(0..9999);
		let source = format!("  int 	main() \n{{ return  {};   \n}}", rand_num);
		let mut scanner = Scanner::new(source.as_str());
		let tokens = scanner.scan_token().expect("scan token failed");
		assert_eq!(tokens, vec![
			TOKEN::Int,
			TOKEN::IDENTIFIER("main".to_string()),
			TOKEN::LeftParen,
			TOKEN::RightParen,
			TOKEN::LeftBrace,
			TOKEN::Return,
			TOKEN::CONST(rand_num),
			TOKEN::Semicolon,
			TOKEN::RightBrace
			]);
	}

	#[test]
	fn test_scan_token_invalid() {
		let source = format!("int %main() {{ return 1foo }}");
		let mut scanner = Scanner::new(source.as_str());
		assert_eq!(scanner.scan_token(), Err(
			vec![
				ScanErr {
					line: 1,
					message: "Unexpected character: %".to_string()
				},
				ScanErr {
					line: 1,
					message: "Invalid number".to_string()
				}
			]
		));
	}
}