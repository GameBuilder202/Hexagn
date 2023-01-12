use std::collections::HashMap;
use std::{fs::File, io::Read};

use codegem::ir::{ModuleBuilder, Type, FunctionId, VariableId, Operation, Value, ToIntegerOperation, Terminator};
use codegem::ir::Module;

use crate::unwrap_or_err;
use super::ast::nodes::{HType, Program, Expr};
use super::lexer::tokenize;
use super::ast::{*, nodes::Node};
use super::ast::nodes::Comparison;
use nodes::Operation::*;

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
				functions.insert(name.clone(), builder.new_function(&make_mangled_name(name.clone(), ret_type.clone(), args.clone()), &(hexagn_to_ir_fargs(args)
					.iter()
					.map(|(s, v)| {
						(s.as_str(), v.clone())
					}).collect::<Vec<(&str, Type)>>()), &ret_type.to_ir_type()));
				builder.switch_to_function(functions[&name]);
				let b = builder.push_block().unwrap();
				builder.switch_to_block(b);
				compile(body, builder, functions, variables);
				builder.set_terminator(Terminator::ReturnVoid);
			},
			Node::VarDefineNode { typ, ident, expr } => {
				let val = compile_expr(expr, builder, functions, variables, typ.clone().to_ir_type());
				variables.insert(ident.clone(), builder.push_variable(ident.as_str(), &typ.clone().to_ir_type()).unwrap());
				builder.push_instruction(&typ.to_ir_type(), Operation::SetVar(variables[&ident], val));
			}
			Node::VarAssignNode { ident, expr } => {
				let val = compile_expr(Some(expr), builder, functions, variables, Type::Integer(false, 64));
				builder.push_instruction(&Type::Integer(false, 64), Operation::SetVar(variables[&ident], val));
			}
			Node::IfNode { cond, body } => {
				let cond_val = compile_expr(Some(cond), builder, functions, variables, Type::Integer(false, 64));
				let if_block = builder.push_block().unwrap();
				let if_end = builder.push_block().unwrap();
				builder.set_terminator(Terminator::Branch(cond_val, if_block, if_end));
				builder.switch_to_block(if_block);
				compile(body, builder, functions, variables);
				builder.switch_to_block(if_end);
			}
			_ => todo!("Unimplimented AST node")
		}
	}
}

fn compile_expr(expr: Option<Expr>, builder: &mut ModuleBuilder, functions: &mut HashMap<String, FunctionId>, variables: &mut HashMap<String, VariableId>, typ: Type) -> Value {
	match typ {
		Type::Integer(_, _) => (),
		Type::Void => unreachable!(),
	}
	match expr.unwrap() {
		Expr::Number(n) => {
			builder.push_instruction(&typ, n.to_integer_operation()).unwrap()
		}
		Expr::BiOp { lhs, op, rhs } => {
			let lhs_val = compile_expr(Some((*lhs).clone()), builder, functions, variables, typ.clone());
			let rhs_val = compile_expr(Some((*rhs).clone()), builder, functions, variables, typ.clone());
			match op {
				Add => builder.push_instruction(&typ, Operation::Add(lhs_val, rhs_val)).unwrap(),
				Sub => builder.push_instruction(&typ, Operation::Sub(lhs_val, rhs_val)).unwrap(),
				Mult => builder.push_instruction(&typ, Operation::Mul(lhs_val, rhs_val)).unwrap(),
				Div => builder.push_instruction(&typ, Operation::Div(lhs_val, rhs_val)).unwrap(),
				Mod => builder.push_instruction(&typ, Operation::Mod(lhs_val, rhs_val)).unwrap(),
			}
		}
		Expr::Ident(n) => {
			builder.push_instruction(&typ, Operation::GetVar(variables[&n])).unwrap()
		}
		Expr::Comp { lhs, comp, rhs } => {
			let lhs_val = compile_expr(Some((*lhs).clone()), builder, functions, variables, typ.clone());
			let rhs_val = compile_expr(Some((*rhs).clone()), builder, functions, variables, typ.clone());
			match comp {
				Comparison::EQ => {
					builder.push_instruction(&typ, Operation::Eq(lhs_val, rhs_val)).unwrap()
				}
				Comparison::NEQ => {
					builder.push_instruction(&typ, Operation::Ne(lhs_val, rhs_val)).unwrap()
				}
				Comparison::LT => {
					builder.push_instruction(&typ, Operation::Lt(lhs_val, rhs_val)).unwrap()
				}
				Comparison::LTE => {
					builder.push_instruction(&typ, Operation::Le(lhs_val, rhs_val)).unwrap()
				}
				Comparison::GT => {
					builder.push_instruction(&typ, Operation::Gt(lhs_val, rhs_val)).unwrap()
				}
				Comparison::GTE => {
					builder.push_instruction(&typ, Operation::Ge(lhs_val, rhs_val)).unwrap()
				}
			}
		}
		_ => todo!()
	}
}


fn hexagn_to_ir_fargs(hexagn_args: Vec<(HType, String)>) -> Vec<(String, Type)> {
	hexagn_args.into_iter().map(|(typ, s)| {
		(s, typ.to_ir_type())
	}).collect()
}

fn make_mangled_name(name: String, ret_type: HType, args: Vec<(HType, String)>) -> String {
	let mut args_mangled = String::new();
	for (arg, _) in args {
		args_mangled += &arg.to_string();
	}
	format!("_Hx{}{}{}{}", name.len(), name, ret_type, args_mangled)
}
