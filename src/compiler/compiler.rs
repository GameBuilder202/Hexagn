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
use inkwell::types::{IntType, BasicMetadataTypeEnum};
use inkwell::values::{FunctionValue, IntValue, BasicMetadataValueEnum, PointerValue};

pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub no_main: bool,
}

pub fn compiler(args: &Args) {
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

    Codegen::compile(prog, std::path::Path::new("./out.o"), true);
}


struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    cur_fn: Option<FunctionValue<'ctx>>,
    vars: HashMap<String, PointerValue<'ctx>>
}

impl<'ctx> Codegen<'ctx> {
    pub fn compile(ast: Program, output_path: &Path, emit_ir: bool) {
        let context = Context::create();
        let module = context.create_module("hexagn");
        let mut codegen = Codegen {
            context: &context,
            module,
            builder: context.create_builder(),

            cur_fn: None,
            vars: HashMap::new()
        };
        codegen.compile_ast(ast);
        codegen.module.print_to_stderr();
        codegen.write_object(output_path);
        if emit_ir {
            codegen.module.write_bitcode_to_path(std::path::Path::new("./out.bc"));
        }
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
        self.module.verify().unwrap(); 
        target_machine.write_to_file(&self.module, FileType::Object, path).unwrap(); // can be changed to asm here
    }

    pub fn compile_ast(&mut self, prog: Program) {
        for statement in prog.statements {
            match statement {
                Node::FunctionNode { ret_type, name, args, body, linkage } => {
                    let ty = self.hexagn_to_llvm_type(ret_type).fn_type(&self.args_to_metadata(&args), false);
                    let func = self.module.add_function(&name, ty, linkage);
        
                    self.cur_fn = Some(func);
                    let entry = self.context.append_basic_block(func, "entry");
                    self.builder.position_at_end(entry);
                    self.compile_ast(body);
                },
                Node::ReturnNode { expr } => {
                    let e = self.compile_expr(expr);
                    self.builder.build_return(Some(&e));
                },
                Node::ExternNode { name, args, ret_type } => {
                    let typ = self.hexagn_to_llvm_type(ret_type).fn_type(&self.args_to_metadata(&args), false);
                    self.module.add_function(&name, typ, Some(Linkage::External));
                },
                Node::FuncCallNode { name, args } => {
                    let func = self.module.get_function(&name).expect("Undefined function. note: functions must be defined before they are called.");
                    self.builder.build_call(func, &self.args_to_value(args), &name);
                },
                Node::VarDefineNode { typ, ident, expr } => {
                    let alloca = self.builder.build_alloca(self.hexagn_to_llvm_type(typ), "buildvar");
                    self.vars.insert(ident, alloca);
                    let val = self.compile_expr(expr.unwrap());
                    self.builder.build_store(alloca, val);
                },
                _ => todo!()
            }
        }
    }

    fn compile_expr(&self, expr: Expr) -> IntValue<'ctx> {
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
                }
            },
            Expr::Number(n) => {
                self.context.i64_type().const_int(n as u64, false)
            },
            Expr::Ident(i) => {
                let var = self.vars[&i];
                self.builder.build_load(self.context.i64_type(), var, "loadvar").into_int_value()
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
                    "void" => self.context.i8_type(),
                    _ => todo!()
                }
            }
            _ => todo!()
        }
    }

    fn args_to_metadata(&self, args: &[(HType, String)]) -> Vec<BasicMetadataTypeEnum<'ctx>> {
        let mut ret = Vec::new();
        for (typ, _) in args {
            match typ.clone() {
                HType::Named(n) => {
                    if n.contains("int") {
                        ret.push(BasicMetadataTypeEnum::IntType(self.hexagn_to_llvm_type((*typ).clone())));
                    }
                }
                _ => todo!()
            }
        }
        ret
    }
    
    fn args_to_value(&self, args: Vec<Expr>) -> Vec<BasicMetadataValueEnum> {
        let mut ret = Vec::new();
        for arg in args {
            let val = self.compile_expr(arg);
            ret.push(BasicMetadataValueEnum::IntValue(val));
        }
        ret
    }
}

