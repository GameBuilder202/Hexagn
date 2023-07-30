use std::collections::VecDeque;
use std::fmt::Write;
use std::process::exit;

use super::{
    super::linker::{Linker, LinkerFunc},
    nodes::*,
};

pub fn compile_ast(prog: &Program, compile_args: AstCompileArgs, linker: &mut Linker) -> Result<String, std::fmt::Error> {
    internal_compile_ast(prog, compile_args, linker, &VarStack::new(), &None)
}

fn internal_compile_ast(
    prog: &Program,
    compile_args: AstCompileArgs,
    linker: &mut Linker,
    outer_scope: &VarStack,
    func_args: &Option<VarStack>,
) -> Result<String, std::fmt::Error> {
    let mut out = String::new();
    let mut var_stack = outer_scope.clone();
    var_stack.push_frame();

    if !compile_args.standalone {
        writeln!(out, "BITS == 32")?;
        writeln!(out, "MINHEAP 4096")?;
        writeln!(out, "MINSTACK 1024")?;
        writeln!(out, "CAL ._Hx4maini8")?;
        writeln!(out, "HLT\n")?;
    }

    for (sym, stmt) in &prog.statements {
        match stmt {
            Node::VarDefine { typ, ident, expr } => {
                if compile_args.debug_symbols {
                    writeln!(out, "// {}: {}", sym.lineno, sym.val)?
                }

                var_stack.push(ident.to_string(), typ);
                if let Some(expr) = expr {
                    write!(out, "{}", compile_expr(expr, linker, &var_stack, func_args, 32).unwrap())?
                } else {
                    writeln!(out, "DEC SP SP\n")?
                }
            }

            Node::VarAssign { ident, expr } => {
                if compile_args.debug_symbols {
                    writeln!(out, "// {}: {}", sym.lineno, sym.val)?
                }

                match expr {
                    Expr::Number(num) => writeln!(out, "IMM R2 {}", num)?,

                    _ => write!(out, "{}", compile_expr(expr, linker, &var_stack, func_args, 32).unwrap())?,
                }
                if let Some(offset) = var_stack.get_offset(ident) {
                    writeln!(out, "LSTR R1 -{} R2", offset)?
                }
            }

            Node::Function { ret_type, name, args, body } => {
                let code = internal_compile_ast(
                    body,
                    AstCompileArgs {
                        debug_symbols: compile_args.debug_symbols,
                        standalone: true,
                        pop_frame: false,
                    },
                    linker,
                    &var_stack,
                    func_args,
                )
                .unwrap();

                let func = LinkerFunc::new(ret_type, name, &args.iter().map(|arg| arg.0.clone()).collect::<Vec<_>>(), &code);

                linker.add_func(&func)
            }

            Node::FuncCall { name, args } => {
                if compile_args.debug_symbols {
                    writeln!(out, "// {}: {}", sym.lineno, sym.val)?
                }

                fn get_arg_types(args: &Vec<Expr>, var_stack: &VarStack, func_args: &Option<VarStack>, sym: &DebugSym) -> Vec<Type> {
                    let mut res = Vec::new();
                    for arg in args {
                        match arg {
                            Expr::Number(_) => res.push(Type::Named(String::from("int"))),

                            Expr::Ident(ident) => {
                                let var_type;
                                if let Some(typ) = var_stack.get_type(ident) {
                                    var_type = typ
                                } else if let Some(func_args) = func_args {
                                    if let Some(typ) = func_args.get_type(ident) {
                                        var_type = typ
                                    } else {
                                        eprintln!("Error: Undefined variable {} at line {}", ident, sym.lineno);
                                        eprintln!("{}: {}", sym.lineno, sym.val);
                                        exit(1)
                                    }
                                } else {
                                    eprintln!("Error: Undefined variable {} at line {}", ident, sym.lineno);
                                    eprintln!("{}: {}", sym.lineno, sym.val);
                                    exit(1)
                                }
                                res.push(var_type)
                            }

                            _ => todo!(),
                        }
                    }
                    res
                }
                let arg_types = get_arg_types(args, &var_stack, func_args, sym);
                let func = linker.get_func(name, &arg_types);
                if let Some(func) = func {
                    for arg in args {
                        write!(out, "{}", compile_expr(arg, linker, &var_stack, func_args, 32).unwrap())?
                    }
                    writeln!(out, "CAL .{}", func.get_signature())?;

                    let args_len = args.len();
                    if args_len > 0 {
                        writeln!(out, "ADD SP SP {}", args_len)?
                    }
                } else {
                    eprintln!("Error: Undefined function {} at line {}", name, sym.lineno);
                    eprintln!("{}: {}", sym.lineno, sym.val);
                    exit(1)
                }
            }

            Node::Return(expr) => {
                if let Some(expr) = expr {
                    write!(out, "{}", compile_expr(expr, linker, &var_stack, func_args, 32).unwrap())?
                }
                // cdecl exit
                writeln!(out, "MOV SP R1")?;
                writeln!(out, "POP R1\n")?;
                writeln!(out, "RET")?
            }

            Node::InlineURCL(urcl) => {
                if compile_args.debug_symbols {
                    writeln!(out, "// Inline URCL @ line {}", sym.lineno)?
                }
                writeln!(out, "{}\n", urcl)?
            }

            _ => todo!(),
        }
    }

    let frames = var_stack.pop_frame();
    if compile_args.pop_frame && frames > 0 {
        writeln!(out, "ADD SP SP {}", frames)?
    }

    if !compile_args.standalone {
        for func in linker.get_funcs() {
            writeln!(out, ".{}", func.get_signature())?;

            // cdecl entry
            writeln!(out, "PSH R1")?;
            writeln!(out, "MOV R1 SP\n")?;

            writeln!(out, "{}", func.code)?;

            // cdecl exit
            writeln!(out, "MOV SP R1")?;
            writeln!(out, "POP R1\n")?;
            writeln!(out, "RET")?
        }
    }

    Ok(out)
}

fn compile_expr(expr: &Expr, _linker: &mut Linker, vars: &VarStack, func_args: &Option<VarStack>, max: u32) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    let max = 1u64.checked_shl(max).unwrap_or(0).wrapping_sub(1);

    match expr {
        Expr::Number(num) => writeln!(s, "PSH {}\n", (*num as u64) % max)?,

        Expr::Ident(name) => match vars.get_offset(name) {
            Some(offset) => {
                writeln!(s, "LLOD R2 R1 -{}", offset)?;
                writeln!(s, "PSH R2")?
            }

            None => {
                todo!()
            }
        },

        Expr::Str(_) => todo!(),

        Expr::BiOp { lhs: _, op: _, rhs: _ } => {
            fn get_op_str(op: &Operation) -> &'static str {
                match op {
                    Operation::Add => "ADD",
                    Operation::Sub => "SUB",
                    Operation::Mult => "MLT",
                    Operation::Div => "DIV",
                    Operation::Mod => "MOD",
                }
            }
            fn compile_expr_recursive(
                expr: &Expr,
                reg_count: u64,
                vars: &VarStack,
                func_args: &Option<VarStack>,
                instr_queue: &mut VecDeque<String>,
            ) -> Result<String, std::fmt::Error> {
                let mut ret = String::new();
                let mut reg_count = reg_count;

                if let Expr::BiOp { lhs, op, rhs } = expr {
                    write!(ret, "{} R{} ", get_op_str(op), reg_count)?;

                    // LHS
                    {
                        let lhs1 = lhs;
                        match *lhs1.clone() {
                            Expr::Number(num) => write!(ret, "{} ", num)?,

                            Expr::Ident(name) => {
                                if let Some(offset) = vars.get_offset(&name) {
                                    instr_queue.push_back(format!(
                                        "LLOD R{} R1 -{}\n",
                                        {
                                            reg_count += 1;
                                            reg_count
                                        },
                                        offset
                                    ))
                                } else if let Some(func_args) = func_args {
                                    if let Some(offset) = func_args.get_offset(&name) {
                                        instr_queue.push_back(format!(
                                            "LLOD R{} R1 {}\n",
                                            {
                                                reg_count += 1;
                                                reg_count
                                            },
                                            offset + 2
                                        ))
                                    }
                                } else {
                                    eprintln!("Error: Undefined variable {}", name);
                                    exit(1)
                                }
                                write!(ret, "R{} ", reg_count)?
                            }

                            Expr::Str(_) => todo!(),

                            Expr::BiOp { lhs: _, op: _, rhs: _ } => {
                                let code = compile_expr_recursive(
                                    lhs1,
                                    {
                                        reg_count += 1;
                                        reg_count
                                    },
                                    vars,
                                    func_args,
                                    instr_queue,
                                )
                                .unwrap();
                                instr_queue.push_back(code);
                                write!(ret, "R{} ", reg_count)?
                            }

                            _ => todo!(),
                        }
                    }

                    // RHS
                    {
                        let rhs1 = rhs;
                        match *rhs1.clone() {
                            Expr::Number(num) => writeln!(ret, "{}", num)?,

                            Expr::Ident(name) => {
                                if let Some(offset) = vars.get_offset(&name) {
                                    instr_queue.push_back(format!(
                                        "LLOD R{} R1 -{}\n",
                                        {
                                            reg_count += 1;
                                            reg_count
                                        },
                                        offset
                                    ))
                                } else if let Some(func_args) = func_args {
                                    if let Some(offset) = func_args.get_offset(&name) {
                                        instr_queue.push_back(format!(
                                            "LLOD R{} R1 {}\n",
                                            {
                                                reg_count += 1;
                                                reg_count
                                            },
                                            offset + 2
                                        ))
                                    }
                                } else {
                                    eprintln!("Error: Undefined variable {}", name);
                                    exit(1)
                                }
                                writeln!(ret, "R{}", reg_count)?
                            }

                            Expr::Str(_) => todo!(),

                            Expr::BiOp { lhs: _, op: _, rhs: _ } => {
                                let code = compile_expr_recursive(
                                    rhs1,
                                    {
                                        reg_count += 1;
                                        reg_count
                                    },
                                    vars,
                                    func_args,
                                    instr_queue,
                                )
                                .unwrap();
                                instr_queue.push_back(code);
                                writeln!(ret, "R{}", reg_count)?
                            }

                            _ => todo!(),
                        }
                    }
                }

                Ok(ret)
            }

            let mut instr_queue = VecDeque::<String>::new();
            let expr_str = compile_expr_recursive(expr, 2, vars, func_args, &mut instr_queue).unwrap();

            while !instr_queue.is_empty() {
                let code = instr_queue.pop_front().unwrap();
                write!(s, "{}", code)?;
            }

            write!(s, "{}", expr_str)?;
            writeln!(s, "AND R2 R2 0x{:x}", max)?;
            writeln!(s, "PSH R2\n")?;
        }

        _ => todo!(),
    }

    Ok(s)
}

#[derive(Clone, Copy)]
pub struct AstCompileArgs {
    pub debug_symbols: bool,
    pub standalone: bool,
    pub pop_frame: bool,
}

#[derive(Clone)]
struct Variable {
    pub name: String,
    pub typ: Type,
}

#[derive(Clone)]
struct VarStack {
    vars: Vec<(Variable, u64)>,
    frames: Vec<u64>,
}

impl VarStack {
    pub fn new() -> Self {
        Self {
            vars: vec![],
            frames: vec![],
        }
    }

    pub fn push(&mut self, name: String, typ: &Type) {
        self.vars.push((Variable { name, typ: typ.clone() }, (self.vars.len() + 1) as u64));
        let len = self.frames.len() - 1;
        self.frames[len] += 1
    }

    fn pop(&mut self, num: u64) {
        for _ in 0..num {
            self.vars.pop();
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(0)
    }

    pub fn pop_frame(&mut self) -> u64 {
        let len = self.frames.len() - 1;
        let frame_count = self.frames[len];
        self.pop(frame_count);
        frame_count
    }

    pub fn get_offset(&self, name: &String) -> Option<u64> {
        for var in &self.vars {
            if var.0.name == *name {
                return Some(var.1);
            }
        }
        None
    }

    pub fn get_type(&self, name: &String) -> Option<Type> {
        for (var, _) in &self.vars {
            if var.name == *name {
                return Some(var.typ.clone());
            }
        }
        None
    }
}
