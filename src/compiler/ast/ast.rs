use std::process::exit;

use crate::{compiler::{lexer::{Token, TokenType}, print_error}, buf_consume, unwrap_or_err};
use super::nodes::*;

pub fn make_ast(src: &String, toks: &Vec<Token>) -> Program
{
	let mut prog = Program::new();
	let mut buf = TokenBuffer::new(src, toks);

	while buf.in_bounds()
	{
		let current = buf.current("");
		match current.tok_type
		{
			// Variable def or function def
			TokenType::Void | TokenType::Int | TokenType::Uint | TokenType::Float | TokenType::String | TokenType::Char => {
				// Making the type
				let mut var_type = Type::Named(current.val.clone());
				while buf.in_bounds()
				{
					let curr = buf.current("");
					if curr.tok_type == TokenType::Identifier { break; }
					if curr.tok_type == TokenType::Mult { var_type = Type::Ptr(Box::new(var_type)) }
					buf.advance()
				}
				buf.advance();
				// let ident = buf_consume!(buf, TokenType::Identifier, src, "Expected identifier after type");
			}

			_ => {
				print_error("Unexpected token", src, current.start, current.end, current.lineno);
				exit(2)
			}
		}
	}

	prog
}

struct TokenBuffer
{
	src:  String,
	toks: Vec<Token>,
	pos:  usize
}

impl TokenBuffer
{
	pub fn new(src: &String, toks: &Vec<Token>) -> TokenBuffer
	{
		TokenBuffer { src: src.to_string(), toks: toks.to_vec(), pos: 0 }
	}

	pub fn in_bounds(&self) -> bool
	{
		self.pos < self.toks.len()
	}

	pub fn advance(&mut self)
	{
		self.pos += 1
	}

	pub fn next(&mut self, err: &str) -> &Token
	{
		self.advance();
		self.current(err)
	}

	pub fn current(&self, err: &str) -> &Token
	{
		let tmp = &self.toks[self.pos - 1];
		unwrap_or_err!(&self.toks.get(self.pos), (self.src, tmp.start, tmp.end, tmp.lineno, err))
	}
}

#[macro_export]
macro_rules! buf_consume {
	($buf:expr, $p:pat, $src:ident, $err:expr) => {
		{
			let curr = $buf.current($err);
			match curr.tok_type {
				$p => { $buf.advance(); curr },
				_ => {
					print_error($err, $src, curr.start, curr.end, curr.lineno);
					exit(2)
				}
			}
		}
	};
}
