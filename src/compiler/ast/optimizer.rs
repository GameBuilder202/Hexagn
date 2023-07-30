use std::process::exit;

use super::nodes::*;

pub fn optimize(prog: &mut Program) {
    for (sym, stmt) in &mut prog.statements {
        match stmt {
            Node::VarDefine { typ: _, ident: _, expr } => {
                if let Some(expr) = expr {
                    *expr = optimize_expr(sym, expr)
                }
            }

            Node::Function {
                ret_type: _,
                name: _,
                args: _,
                body,
            } => optimize(body),

            Node::FuncCall { name: _, args } => {
                for arg in args {
                    *arg = optimize_expr(sym, arg)
                }
            }

            _ => {}
        }
    }
}

fn optimize_expr(sym: &DebugSym, expr: &Expr) -> Expr {
    match expr {
        Expr::BiOp { lhs, op, rhs } => {
            let lhs_opt = optimize_expr(sym, lhs);
            let rhs_opt = optimize_expr(sym, rhs);

            match op {
                Operation::Add => {
                    if let Expr::Number(val1) = lhs_opt {
                        if let Expr::Number(val2) = rhs_opt {
                            Expr::Number(val1 + val2)
                        } else {
                            expr.clone()
                        }
                    } else if let Expr::Str(string) = lhs_opt {
                        match rhs_opt {
                            Expr::Number(val) => Expr::Str(format!("{}{}", string, val)),
                            Expr::Str(string2) => Expr::Str(string + string2.as_str()),
                            _ => {
                                eprintln!("Cannot perform string concatenation at line {}", sym.lineno);
                                eprintln!("{}: {}", sym.lineno, sym.val);
                                exit(1)
                            }
                        }
                    } else {
                        expr.clone()
                    }
                }

                Operation::Sub => {
                    if let Expr::Number(val1) = lhs_opt {
                        if let Expr::Number(val2) = rhs_opt {
                            Expr::Number(val1 - val2)
                        } else {
                            expr.clone()
                        }
                    } else {
                        expr.clone()
                    }
                }

                Operation::Mult => {
                    if let Expr::Number(val1) = lhs_opt {
                        if let Expr::Number(val2) = rhs_opt {
                            Expr::Number(val1 * val2)
                        } else {
                            expr.clone()
                        }
                    } else {
                        expr.clone()
                    }
                }

                Operation::Div => {
                    if let Expr::Number(val1) = lhs_opt {
                        if let Expr::Number(val2) = rhs_opt {
                            if val2 == 0 {
                                eprintln!("Error: Division by 0 after constant folding at line {}", sym.lineno);
                                eprintln!("{}: {}", sym.lineno, sym.val);
                                exit(1)
                            }
                            Expr::Number(val1 / val2)
                        } else {
                            expr.clone()
                        }
                    } else {
                        expr.clone()
                    }
                }

                Operation::Mod => {
                    if let Expr::Number(val1) = lhs_opt {
                        if let Expr::Number(val2) = rhs_opt {
                            Expr::Number(val1 % val2)
                        } else {
                            expr.clone()
                        }
                    } else {
                        expr.clone()
                    }
                }
            }
        }

        other => other.clone(),
    }
}
