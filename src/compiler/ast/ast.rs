use std::process::exit;

use super::nodes::*;
use crate::{
    buf_consume,
    compiler::{
        draw_arrows,
        lexer::{Token, TokenType},
        print_error,
    },
    unwrap_or_err,
};

pub fn make_ast(src: &String, toks: &Vec<Token>) -> Program {
    let mut prog = Program::new();
    let mut buf = TokenBuffer::new(src, toks);

    while buf.in_bounds() {
        let current = buf.current("").clone();
        match current.tok_type {
            // Variable def or function def
            TokenType::Void | TokenType::Int | TokenType::Uint | TokenType::Float | TokenType::String | TokenType::Char => {
                // Making the type
                let var_type = make_type(&mut buf);
                let ident = buf_consume!(buf, (TokenType::Identifier), src, "Expected identifier after type");
                let op = buf_consume!(
                    buf,
                    (TokenType::Assign, TokenType::OpenParen, TokenType::Semicolon),
                    src,
                    "Expected '=' or '(' or ';' after identifier"
                );

                match op.tok_type {
                    // Variable definition
                    TokenType::Assign => {
                        if current.tok_type == TokenType::Void {
                            print_error("Cannot have void for variable type", src, current.start, current.end, current.lineno);
                            exit(2)
                        }
                        if !buf.in_bounds() {
                            print_error("Expected expression after '='", src, op.start, op.end, op.lineno);
                            exit(2)
                        }
                        let expr = expr_parser(&mut buf, src);
                        buf_consume!(buf, (TokenType::Semicolon), src, "Expected ';' after expression");
                        prog.statements.push(Node::VarDefineNode {
                            typ: var_type,
                            ident: ident.val,
                            expr: Some(expr),
                        })
                    }

                    // Variable declaration
                    TokenType::Semicolon => {
                        prog.statements.push(Node::VarDefineNode {
                            typ: var_type,
                            ident: ident.val,
                            expr: None,
                        });
                        buf.advance()
                    }

                    // Function definition
                    TokenType::OpenParen => {
                        let mut args = vec![];
                        if !buf.in_bounds() || !(is_datatype(buf.current("")) || buf.current("").tok_type == TokenType::CloseParen) {
                            print_error("Expected type or '(' after ')'", src, op.start, op.end, op.lineno);
                            exit(2)
                        }

                        while buf.in_bounds() && buf.current("").tok_type != TokenType::CloseParen {
                            let arg_type = make_type(&mut buf);
                            let arg_ident = buf_consume!(buf, (TokenType::Identifier), src, "Expected identifier after type");
                            args.push((arg_type, arg_ident.val));

                            if !buf.in_bounds() {
                                print_error("Expected ')' or ',' after identifier", src, arg_ident.start, arg_ident.end, arg_ident.lineno);
                                exit(2)
                            }
                            let curr = buf.current("");
                            if curr.tok_type == TokenType::CloseParen {
                                break;
                            }
                            if curr.tok_type != TokenType::Semicolon {
                                print_error("Expected ')' or ',' after identifier", src, curr.start, curr.end, curr.lineno);
                                exit(2)
                            }
                            buf.advance()
                        }

                        buf.advance();
                        let func_body = sub_program(&mut buf, src, "function body");
                        prog.statements.push(Node::FunctionNode {
                            ret_type: var_type,
                            name: ident.val,
                            args,
                            body: func_body,
                        })
                    }

                    _ => {
                        unreachable!()
                    }
                }
            }

            TokenType::If => {
                buf.advance();
                let expr = expr_parser(&mut buf, src);
                let body = sub_program(&mut buf, src, "if statement");
                prog.statements.push(Node::IfNode { cond: expr, body })
            }

            TokenType::While => {
                buf.advance();
                let expr = expr_parser(&mut buf, src);
                let body = sub_program(&mut buf, src, "while statement");
                prog.statements.push(Node::WhileNode { cond: expr, body })
            }

            TokenType::Import => {
                let mut lib_name = String::new();

                buf.advance();
                while buf.in_bounds() && buf.current("").tok_type != TokenType::Semicolon {
                    lib_name += buf_consume!(buf, (TokenType::Identifier, TokenType::URCLBlock), src, "Expected module name").val.as_str();
                    if buf.current("Expected '.' or ':' after module name").tok_type == TokenType::Semicolon {
                        break;
                    }
                    lib_name += buf_consume!(buf, (TokenType::Dot, TokenType::Colon), src, "Expected '.' or ':' after module name").val.as_str();

                    buf.advance()
                }
                buf.advance();
                prog.statements.push(Node::ImportNode(lib_name))
            }

            TokenType::URCLBlock => {
                buf.advance();
                let urcl = buf_consume!(buf, (TokenType::Str), src, "Expected URCL code in string after keyword").val;
                buf_consume!(buf, (TokenType::Semicolon), src, "Expected ';' after URCL block string");
                prog.statements.push(Node::URCLBlockNode(urcl))
            }

            TokenType::Semicolon => buf.advance(),

            _ => {
                print_error("Unexpected token", src, current.start, current.end, current.lineno);
                exit(2)
            }
        }
    }

    prog
}

struct TokenBuffer {
    src: String,
    toks: Vec<Token>,
    pos: usize,
}

impl TokenBuffer {
    pub fn new(src: &String, toks: &Vec<Token>) -> TokenBuffer {
        TokenBuffer {
            src: src.to_string(),
            toks: toks.to_vec(),
            pos: 0,
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.pos < self.toks.len()
    }

    pub fn advance(&mut self) {
        self.pos += 1
    }
    #[allow(dead_code)]
    pub fn next(&mut self, err: &str) -> &Token {
        self.advance();
        self.current(err)
    }

    pub fn current(&self, err: &str) -> &Token {
        let tmp = if self.pos != 0 { &self.toks[self.pos - 1] } else { &self.toks[self.pos] };
        unwrap_or_err!(&self.toks.get(self.pos), (self.src, tmp.start, tmp.end, tmp.lineno, err))
    }
}

#[macro_export]
macro_rules! buf_consume {
    ($buf:ident, ($($p:pat),+), $src:ident, $err:expr) => {
        {
            let curr = $buf.current($err).clone();
            match curr.tok_type {
                $($p)|+ => { $buf.advance(); curr },
                _ => {
                    print_error($err, $src, curr.start, curr.end, curr.lineno);
                    draw_arrows(curr.start, curr.end, curr.lineno);
                    exit(2)
                }
            }
        }
    };
}

fn make_type(buf: &mut TokenBuffer) -> Type {
    let mut var_type = Type::Named(buf.current("").val.clone());
    while buf.in_bounds() {
        let curr = buf.current("");
        if curr.tok_type == TokenType::Identifier {
            return var_type;
        }
        if curr.tok_type == TokenType::Mult {
            var_type = Type::Ptr(Box::new(var_type))
        }
        buf.advance()
    }
    var_type
}

fn expr_parser(buf: &mut TokenBuffer, src: &String) -> Expr {
    fn factor(buf: &mut TokenBuffer, src: &String) -> Expr {
        let tok = buf_consume!(
            buf,
            (TokenType::Num, TokenType::Identifier, TokenType::Str, TokenType::OpenParen),
            src,
            "Expected number or identifier or string or open paren"
        );
        match tok.tok_type {
            TokenType::Num => Expr::Number(tok.val.parse::<i64>().unwrap()),
            TokenType::Identifier => {
                if buf.current("Expected operation or '(' or ';' after identifier").tok_type == TokenType::OpenParen {
                    todo!("Make Functions stuff")
                }
                Expr::Ident(tok.val)
            }
            TokenType::String => Expr::Str(tok.val),
            TokenType::OpenParen => {
                let node = expr(buf, src);
                println!("{:#?}", buf.current(""));
                buf_consume!(buf, (TokenType::CloseParen), src, "Missing closing ')'");
                node
            }
            other => {
                println!("{:#?}", other);
                unreachable!()
            }
        }
    }

    fn tok_to_op(tok: &Token) -> Operation {
        match tok.tok_type {
            TokenType::Plus => Operation::Add,
            TokenType::Minus => Operation::Sub,
            TokenType::Mult => Operation::Mult,
            TokenType::Div => Operation::Div,
            TokenType::Mod => Operation::Mod,
            _ => {
                unreachable!()
            }
        }
    }

    fn term(buf: &mut TokenBuffer, src: &String) -> Expr {
        let mut node = factor(buf, src);
        while buf.current("Expected operation").tok_type == TokenType::Mult || buf.current("").tok_type == TokenType::Div {
            let op = buf.current("").clone();
            buf.advance();
            node = Expr::BiOp {
                lhs: Box::new(node),
                op: tok_to_op(&op),
                rhs: Box::new(factor(buf, src)),
            };
        }
        node
    }

    fn expr(buf: &mut TokenBuffer, src: &String) -> Expr {
        let mut node = term(buf, src);
        while buf.current("Expected operation").tok_type == TokenType::Plus || buf.current("").tok_type == TokenType::Minus {
            let op = buf.current("").clone();
            buf.advance();
            node = Expr::BiOp {
                lhs: Box::new(node),
                op: tok_to_op(&op),
                rhs: Box::new(term(buf, src)),
            };
        }
        node
    }

    expr(buf, src)
}

fn is_datatype(tok: &Token) -> bool {
    tok.tok_type == TokenType::Void
        || tok.tok_type == TokenType::Int
        || tok.tok_type == TokenType::Uint
        || tok.tok_type == TokenType::Float
        || tok.tok_type == TokenType::String
        || tok.tok_type == TokenType::Character
}

fn sub_program(buf: &mut TokenBuffer, src: &String, err: &str) -> Program {
    buf_consume!(buf, (TokenType::OpenBrace), src, format!("Expected '{{' for {}", err).as_str());
    let mut body = vec![];
    let mut scope = 0;
    while buf.in_bounds() {
        let curr = buf.current("").clone();
        if curr.tok_type == TokenType::OpenBrace {
            scope += 1
        } else if curr.tok_type == TokenType::CloseBrace {
            if scope == 0 {
                break;
            }
            scope -= 1;
        }
        body.push(curr);
        buf.advance()
    }
    buf_consume!(buf, (TokenType::CloseBrace), src, format!("Expected '}}' for {}", err).as_str());

    make_ast(src, &body)
}
