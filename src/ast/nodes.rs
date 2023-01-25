#![allow(dead_code)]

use std::fmt::Display;

use inkwell::{module::Linkage, types::{FunctionType}};


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
    VarDefineNode { // TODO: Add compilation for this
        typ: HType,
        ident: String,
        expr: Option<Expr>,
    },
    VarAssignNode { // TODO: Add compilation
        ident: String,
        expr: Expr,
    },
    FunctionNode { // DONE
        ret_type: HType,
        name: String,
        args: Vec<(HType, String)>,
        body: Program,
        linkage: Option<Linkage>
    },
    FuncCallNode { // DONE
        name: String,
        args: Vec<Expr>,
    },
    WhileNode { // TODO: add compilation and proper expr parsing
        cond: Expr,
        body: Program,
    },
    IfNode { // TODO: add compilation and proper expr parsing for this
        cond: Expr,
        body: Program,
    },
    ImportNode(String), // TODO
    URCLBlockNode(String), // probably never gonna be added (LLVM doesnt support URCL)
    ExternNode { // DONE
        name: String,
        args: Vec<(HType, String)>,
        ret_type: HType
    },
    ReturnNode { // DONE
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

impl<'a> HType {
    pub fn to_fn_type(self) -> FunctionType<'a> {
        match self {
            HType::Named(n) => {
                match n {
                    _ => todo!()
                }
            }
            _ => todo!()
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
