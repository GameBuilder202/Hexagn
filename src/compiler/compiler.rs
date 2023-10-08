use std::{fs::File, io::Read};

use super::lexer::tokenize;
use super::linker::Linker;
use super::{
    ast::{
        ast_compiler::{compile_ast, AstCompileArgs},
        make_ast, optimizer,
    },
    imports::ImportHelper,
};
use crate::unwrap_or_err;

pub struct Args {
    pub input_file: String,
    pub no_main: bool,
    pub debug_symbols: bool,
    pub opt_level: u32,
}

pub fn compiler(args: &Args, linker: &mut Linker, importer: &mut ImportHelper) -> String {
    let mut src = String::new();

    let mut input_file = unwrap_or_err!(File::open(&args.input_file), "Unable to open input file");

    unwrap_or_err!(input_file.read_to_string(&mut src), "Could not read input file");
    src = format!("\n{}", src);

    let toks = tokenize(&src);
    // println!("{:#?}", toks);

    // println!("#------------------------#");

    let mut prog = make_ast(&src, &toks);
    for _ in 0..args.opt_level {
        optimizer::optimize(&mut prog)
    }
    // println!("{:#?}", prog);

    compile_ast(
        &prog,
        AstCompileArgs {
            debug_symbols: args.debug_symbols,
            standalone: args.no_main,
            pop_frame: false,
            opt_level: args.opt_level,
        },
        linker,
        importer,
    )
    .unwrap()
}
