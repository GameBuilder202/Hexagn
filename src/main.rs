use clap::{ArgAction, Parser};

mod compiler;
use compiler::*;

pub mod util;

fn main() {
    let args = Args::parse();
    for _lib in args.lib_paths {
        todo!()
    }

    let prog = compiler(&compiler::Args {
        input_file: args.input_file,
        output_file: args.output_file,
        no_main: args.no_main,
    });
}

#[derive(Parser)]
struct Args {
    #[clap(value_name = "Input file")]
    input_file: String,

    #[clap(short, name = "o", value_name = "Output file", default_value = "./out.urcl")]
    output_file: String,

    #[clap(short, name="l", value_name="Library path", action=ArgAction::Append)]
    lib_paths: Vec<String>,

    #[clap(long="no-main", value_name="Emit entry point", action=ArgAction::Set, default_value="false")]
    no_main: bool,
}
