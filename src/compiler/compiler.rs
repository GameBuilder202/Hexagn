use std::collections::HashMap;
use std::{fs::File, io::Read};

use codegem::ir::{ModuleBuilder, Type, FunctionId, VariableId, Operation, Value, ToIntegerOperation};
use codegem::ir::Module;

use crate::unwrap_or_err;
use super::ast::nodes::{HType, Program, Expr};
use super::lexer::tokenize;
use super::ast::{*, nodes::Node};

pub struct Args
{
	pub input_file: String,
	pub output_file: String,
	pub no_main: bool
}

pub fn compiler(args: &Args) -> Module
{
	let mut src = String::new();

	let mut input_file = unwrap_or_err!(File::open(&args.input_file), "Unable to open input file");

	unwrap_or_err!(input_file.read_to_string(&mut src), "Could not read input file");
	src = format!("\n{}", src);

	let toks = tokenize(&src);
	println!("{:#?}", toks);

	println!("#------------------------#");

	let prog = make_ast(&src, &toks);
	println!("{:#?}", prog);

    let mut builder = ModuleBuilder::default().with_name("hexagn");
    let mut functions: HashMap<String, FunctionId> = HashMap::new();
    let mut variables: HashMap<String, VariableId> = HashMap::new();

    compile(prog, &mut builder, &mut functions, &mut variables);

    builder.build()
}

fn compile(prog: Program, builder: &mut ModuleBuilder, functions: &mut HashMap<String, FunctionId>, variables: &mut HashMap<String, VariableId>) {
    for statement in prog.statements {
        match statement {
            Node::FunctionNode { ret_type, name, args, body } => {
                functions.insert(name.clone(), builder.new_function(&name, &[], &ret_type.to_ir_type()));
                builder.switch_to_function(functions[&name]);
                compile(body, builder, functions, variables);
            },
            Node::VarDefineNode { typ, ident, expr } => {
                let val = compile_expr(expr, builder, functions, variables, typ.clone().to_ir_type());
                variables.insert(ident.clone(), builder.push_variable(ident.as_str(), &typ.clone().to_ir_type()).unwrap());
                builder.push_instruction(&typ.to_ir_type(), Operation::SetVar(variables[&ident], val));
            }
            _ => todo!("Unimplimented AST node")
        }
    }
}

fn compile_expr(expr: Option<Expr>, builder: &mut ModuleBuilder, functions: &mut HashMap<String, FunctionId>, variables: &mut HashMap<String, VariableId>, typ: Type) -> Value {
    let mut s = false;
    let mut v = 0;
    match typ {
        Type::Integer(su, vu) => {
            s = su;
            v = vu;
        }
        Type::Void => unreachable!(),
    }
    match expr.unwrap() {
        Expr::Number(n) => {
            builder.push_instruction(&typ, n.to_integer_operation()).unwrap()
        }
        _ => todo!()
    }
}

impl HType {
    fn to_ir_type(self) -> Type {
        match self {
            HType::Named(n) => {
                match n.as_str() {
                    "int32" => {
                        Type::Integer(true, 32)
                    },
                    "int16" => {
                        Type::Integer(true, 16)
                    },
                    "int8" => {
                        Type::Integer(true, 8)
                    },
                    "uint32" => {
                        Type::Integer(false, 32)
                    },
                    "uint16" => {
                        Type::Integer(false, 16)
                    },
                    "uint8" => {
                        Type::Integer(false, 8)
                    }
                    _ => todo!("Unimplimented type."),
                }
            },
            _ => todo!("Unimplimented type."),
        }
    }
}

//fn hexagn_to_ir_fargs(hexagn_args: Vec<(String, HType)>) -> &[(&'static str, Type)] {
//    // TODO
//}