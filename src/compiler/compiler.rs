use std::collections::HashMap;
use std::{fs::File, io::Read};

use crate::ast::{make_ast, nodes::Program};
use crate::lexer::tokenize;
use crate::unwrap_or_err;

use std::path::Path;

use crate::ast::nodes::{Expr, Operation, HType, Node};

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::types::IntType;
use inkwell::values::{FunctionValue, PointerValue, IntValue};
use inkwell::types::FunctionType;

pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub no_main: bool,
}

pub fn compiler(args: &Args) -> Program {
    let mut src = String::new();

    let mut input_file = unwrap_or_err!(File::open(&args.input_file), "Unable to open input file");

    unwrap_or_err!(
        input_file.read_to_string(&mut src),
        "Could not read input file"
    );
    src = format!("\n{}", src);

    let toks = tokenize(&src);
    println!("{:#?}", toks);

    println!("#------------------------#");

    let prog = make_ast(&src, &toks);
    println!("{:#?}", prog);

    prog
}


struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    cur_fn: Option<FunctionValue<'ctx>>,
    cur_vars: HashMap<String, PointerValue<'ctx>>
}

impl<'ctx> Codegen<'ctx> {
    pub fn compile(ast: Program, output_path: &Path) {
        let context = Context::create();
        let module = context.create_module("hexagn");
        let mut codegen = Codegen {
            context: &context,
            module,
            builder: context.create_builder(),

            cur_fn: None,
            cur_vars: HashMap::new()
        };
        // call smth like compile_ast()
    }

    fn write_object(&self, path: &Path) {
        Target::initialize_x86(&InitializationConfig::default());
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let cpu = TargetMachine::get_host_cpu_name();
        let features = TargetMachine::get_host_cpu_features();
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let opt = OptimizationLevel::Default; // :froggers:
        let target_machine = target
            .create_target_machine(
                &triple, 
                cpu.to_str().unwrap(), 
                features.to_str().unwrap(), 
                opt, 
                reloc, 
                model
            )
            .unwrap();
        
        target_machine.write_to_file(&self.module, FileType::Object, path).unwrap(); // can be changed to asm here
    }

    pub fn compile_ast(&mut self, prog: Program) {
        for statement in prog.statements {
            match statement {
                Node::FunctionNode { ret_type, name, args, body, linkage } => {
                    let ty = todo!();
                    self.module.add_function(&name, ty, Some(linkage));
                }
                _ => todo!()
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> IntValue<'ctx> {
        match expr {
            Expr::BiOp { lhs, op, rhs } => {
                let lhsc = self.compile_expr(*lhs);
                let rhsc = self.compile_expr(*rhs);
                match op {
                    Operation::Add => {
                        self.builder.build_int_add(lhsc, rhsc, "addbiop")
                    },
                    Operation::Sub => {
                        self.builder.build_int_sub(lhsc, rhsc, "subbiop")
                    },
                    Operation::Div => {
                        self.builder.build_int_exact_signed_div(lhsc, rhsc, "divbiop")
                    },
                    Operation::Mult => {
                        self.builder.build_int_mul(lhsc, rhsc, "mulbiop")
                    },
                    Operation::Mod => {
                        self.builder.build_int_signed_rem(lhsc, rhsc, "modbiop")
                    }
                    _ => todo!()
                }
            },
            Expr::Ident(s) => {
                self.builder.build_load(self.cur_vars[&s], "load_var").into_int_value()
            },
            Expr::Number(n) => {
                self.context.i64_type().const_int(n as u64, false)
            }
            _ => todo!()
        }
    }

    fn hexagn_to_llvm_type(&self, typ: HType) -> IntType<'ctx> {
        match typ {
            HType::Named(n) => {
                match n.as_str() {
                    "int8" => self.context.i8_type(),
                    "int16" => self.context.i16_type(),
                    "int32" => self.context.i32_type(),
                    _ => todo!()
                }
            }
            _ => todo!()
        }
    }
}

