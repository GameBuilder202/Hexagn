#![allow(dead_code)]

use std::fmt::Display;

use codegem::ir::{Type, Linkage};
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

impl HType {
    pub fn to_ir_type(self) -> Type {
        match self {
            HType::Named(n) => match n.as_str() {
                "int32" => Type::Integer(true, 32),
                "int16" => Type::Integer(true, 16),
                "int8" => Type::Integer(true, 8),
                "uint32" => Type::Integer(false, 32),
                "uint16" => Type::Integer(false, 16),
                "uint8" => Type::Integer(false, 8),
                "char" => Type::Integer(false, 8),
                "void" => Type::Void,
                _ => todo!("Unimplimented type."),
            },
            _ => todo!("Unimplimented type."),
        }
    }
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
