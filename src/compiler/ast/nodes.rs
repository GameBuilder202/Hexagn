#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<(DebugSym, Node)>,
}
impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[derive(Debug)]
pub struct DebugSym {
    pub val: String,
    pub lineno: usize,
}
impl DebugSym {
    pub fn new(val: String, lineno: usize) -> Self {
        DebugSym { val, lineno }
    }
}

#[derive(Debug)]
pub enum Node {
    VarDefineNode {
        typ: Type,
        ident: String,
        expr: Option<Expr>,
    },
    VarAssignNode {
        ident: String,
        expr: Expr,
    },
    FunctionNode {
        ret_type: Type,
        name: String,
        args: Vec<(Type, String)>,
        body: Program,
    },
    FuncCallNode {
        name: String,
        args: Vec<Expr>,
    },
    WhileNode {
        cond: Expr,
        body: Program,
    },
    IfNode {
        cond: Expr,
        body: Program,
    },
    ImportNode(String),
    InlineURCL(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(String),
    Ptr(Box<Type>),
    Arr(Box<Type>),
    Const(Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(name) => write!(f, "{}", name)?,
            Self::Ptr(typ) => write!(f, "{}*", *typ)?,
            Self::Arr(typ) => write!(f, "{}[]", *typ)?,
            Self::Const(typ) => write!(f, "const {}", *typ)?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    EQ,
    NEQ,
    LT,
    LTE,
    GT,
    GTE,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Str(String),
    BiOp { lhs: Box<Expr>, op: Operation, rhs: Box<Expr> },
    FuncCall { name: String, args: Vec<Expr> },
    Comp { lhs: Box<Expr>, comp: Comparison, rhs: Box<Expr> },
}
