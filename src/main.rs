use std::{collections::HashMap, io::Write};

use clap::{ArgAction, Parser};

mod compiler;
use compiler::*;

pub mod util;

use codegem::{
    arch::{rv64::RvSelector, urcl::UrclSelector, x64::X64Selector},
    ir::ModuleBuilder,
    regalloc::RegAlloc,
};

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

    let mut builder = ModuleBuilder::default().with_name("hexagn");
    let mut functions = HashMap::new();
    let mut variables = HashMap::new();
    compiler::ast::ast_compiler::compile_ast(prog, &mut builder, &mut functions, &mut variables);
    let ir = builder.build();
    let irprint = ir.to_string();
    match args.target.to_lowercase().as_str() {
        "urcl" => {
            let mut vcode = ir.lower_to_vcode::<_, UrclSelector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly(&mut std::fs::File::create("out.urcl").unwrap()).unwrap();
        }
        "rv64" | "riscv" => {
            let mut vcode = ir.lower_to_vcode::<_, RvSelector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly(&mut std::fs::File::create("out.s").unwrap()).unwrap();
        }
        "x86_64" | "x86" | "x64" => {
            let mut vcode = ir.lower_to_vcode::<_, X64Selector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly(&mut std::fs::File::create("out.s").unwrap()).unwrap();
        }
        _ => panic!("Unknown backend"),
    }

    if args.emit_ir {
        let mut file = std::fs::File::create("ir.cgemir").unwrap();
        file.write_all(irprint.as_bytes()).unwrap();
    }
}

#[derive(Parser)]
struct Args {
    #[clap(value_name = "Input file")]
    input_file: String,

    #[clap(
        short,
        name = "o",
        value_name = "Output file",
        default_value = "./out.urcl"
    )]
    output_file: String,

    #[clap(short, name="l", value_name="Library path", action=ArgAction::Append)]
    lib_paths: Vec<String>,

    #[clap(long="no-main", value_name="Emit entry point", action=ArgAction::Set, default_value="false")]
    no_main: bool,

    #[clap(long="target", value_name="Target architecture to compile to", action=ArgAction::Set, default_value="urcl")]
    target: String,

    #[clap(long="emit-ir", value_name="Emit codegem IR", action=ArgAction::Set, default_value="false")]
    emit_ir: bool,
}
