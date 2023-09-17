use std::fs::File;
use std::io::Write;

use clap::{ArgAction, Parser};

mod compiler;
use compiler::{linker::Linker, *};

pub mod util;

fn main() {
    let args = Args::parse();
    for _lib in args.lib_paths {
        todo!()
    }

    let mut main_linker = Linker::new();
    let code = compiler(
        &compiler::Args {
            input_file: args.input_file,
            no_main: args.no_main,
            debug_symbols: args.debug_symbols,
            opt_level: args.opt_level,
        },
        &mut main_linker,
    );

    let mut out_file = File::create(args.output_file).unwrap();
    write!(out_file, "{}", code).unwrap();
}

#[derive(Parser)]
struct Args {
    #[clap(value_name = "Input file")]
    input_file: String,

    #[clap(short, value_name = "Output file", default_value = "./out.urcl")]
    output_file: String,

    #[clap(short, value_name="Library path", action=ArgAction::Append)]
    lib_paths: Vec<String>,

    #[clap(long="no-main", name="Emit entry point", action=ArgAction::SetTrue, default_value="false")]
    no_main: bool,

    #[clap(short='g', name="Debug symbols", action=ArgAction::SetTrue, default_value="false")]
    debug_symbols: bool,

    #[clap(short='O', name="Optimization level", action=ArgAction::Set, default_value="0")]
    opt_level: u32,
}
