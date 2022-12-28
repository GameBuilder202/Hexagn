use std::{fs::File, io::Read};

use crate::{unwrap_or_err, compiler as other_compiler};
use super::lexer::tokenize;
use super::ast::*;

pub struct Args
{
	pub input_file: String,
	pub output_file: String,
	pub no_main: bool
}

pub fn compiler(args: &Args)
{
	let mut src = String::new();

	let mut input_file = unwrap_or_err!(File::open(&args.input_file), "Unable to open input file");

	unwrap_or_err!(input_file.read_to_string(&mut src), "Could not read input file");
	src = format!("\n{}", src);

	let toks = tokenize(&src);
	for tok in &toks
	{
		println!("{:?}", tok);
	}
	let _prog = make_ast(&src, &toks);
}
