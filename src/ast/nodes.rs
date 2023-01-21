#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Linkage {
    Public,
    Private,
    External
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Node>,
}
impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

#[derive(Debug)]
pub enum Node {
    VarDefineNode {
        typ: HType,
        ident: String,
        expr: Option<Expr>,
    },
    VarAssignNode {
        ident: String,
        expr: Expr,
    },
    FunctionNode {
        ret_type: HType,
        name: String,
        args: Vec<(HType, String)>,
        body: Program,
        linkage: Linkage
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
    URCLBlockNode(String),
    ExternNode {
        name: String,
        args: Vec<(HType, String)>,
        ret_type: HType
    },
    ReturnNode {
        expr: Expr
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HType {
    Named(String),
    Ptr(Box<HType>),
    Arr(Box<HType>),
    Const(Box<HType>),
}

impl Display for HType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HType::Named(s) => write!(f, "{}", s),
            HType::Ptr(s) => write!(f, "PTR_{}", *s),
            HType::Const(s) => write!(f, "CONST_{}", *s),
            HType::Arr(s) => write!(f, "ARR_{}", *s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
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
    BiOp {
        lhs: Box<Expr>,
        op: Operation,
        rhs: Box<Expr>,
    },
    Comp {
        lhs: Box<Expr>,
        comp: Comparison,
        rhs: Box<Expr>,
    },
}
