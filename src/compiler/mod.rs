mod compiler;
pub use compiler::*;

use crate::util::{get_line, draw_arrows};
pub mod lexer;
pub mod ast;

pub fn print_error(err: &str, src: &String, start: usize, end: usize, lineno: usize)
{
	eprintln!("Error: {} at line {}", err, lineno);
	eprintln!("{}: {}", lineno, get_line(&src, lineno));
	draw_arrows(start, end, lineno);
}

#[macro_export]
macro_rules! unwrap_or_err
{
	($try:expr, $err:literal) => {
		{
			let res = $try;
			match res {
				Ok(_res) => _res,
				Err(_) => panic!("{:?}", $err)
			}
		}
	};

	($try:expr, ($src:expr, $start:expr, $end:expr, $lineno:expr, $err:ident)) => {
		{
			let res = $try;
			match res {
				Some(_res) => _res,
				None => {
					print_error(&$err, &$src, $start, $end, $lineno);
					exit(1)
				}
			}
		}
	};
}
