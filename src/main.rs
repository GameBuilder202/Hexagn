use std::io::Write;

use clap::{Parser, ArgAction};

mod compiler;
use compiler::*;

pub mod util;

use codegem::{arch::{urcl::UrclSelector, rv64::RvSelector, x64::X64Selector}, regalloc::RegAlloc};

fn main()
{
	let args = Args::parse();
	for _lib in args.lib_paths
	{
		todo!()
	}
    

	let ir = compiler(
		&compiler::Args {
			input_file: args.input_file,
			output_file: args.output_file,
			no_main: args.no_main
		}
	);
    let irprint = ir.to_string();
    match args.target.to_lowercase().as_str() {
        "urcl" => {
            let mut vcode = ir.lower_to_vcode::<_, UrclSelector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly();
        },
        "rv64" | "riscv" => {
            let mut vcode = ir.lower_to_vcode::<_, RvSelector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly();
        },
        "x86_64" | "x86" | "x64" => {
            let mut vcode = ir.lower_to_vcode::<_, X64Selector>();
            vcode.allocate_regs::<RegAlloc>();
            vcode.emit_assembly();
        }
        _ => panic!("Unknown backend")
    }

    if args.emit_ir {
        let mut file = std::fs::File::create("ir.cgemir").unwrap();
        file.write_all(irprint.as_bytes()).unwrap();
    }
}

#[derive(Parser)]
struct Args
{
	#[clap(value_name="Input file")]
	input_file: String,

	#[clap(short, name="o", value_name="Output file", default_value = "./out.urcl")]
	output_file: String,

	#[clap(short, name="l", value_name="Library path", action=ArgAction::Append)]
	lib_paths: Vec<String>,

	#[clap(long="no-main", value_name="Emit entry point", action=ArgAction::Set, default_value="false")]
	no_main: bool,

    #[clap(long="target", value_name="Target architecture to compile to", action=ArgAction::Set, default_value="urcl")]
    target: String,

    #[clap(long="emit-ir", value_name="Emit codegem IR", action=ArgAction::Set, default_value="false")]
    emit_ir: bool
}
