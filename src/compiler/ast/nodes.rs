#![allow(dead_code)]

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
    VarDefineNode { typ: Type, ident: String, expr: Option<Expr> },
    VarAssignNode { ident: String, expr: Expr },
    FunctionNode { ret_type: Type, name: String, args: Vec<(Type, String)>, body: Program },
    FuncCallNode { name: String, args: Vec<Node> },
    WhileNode { cond: Expr, body: Program },
    IfNode { cond: Expr, body: Program },
    ImportNode(String),
    URCLBlockNode(String),
}

#[derive(Debug, Clone)]
pub enum Type {
    Named(String),
    Ptr(Box<Type>),
    Arr(Box<Type>),
    Const(Box<Type>),
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
    BiOp { lhs: Box<Expr>, op: Operation, rhs: Box<Expr> },
    Comp { lhs: Box<Expr>, comp: Comparison, rhs: Box<Expr> },
}
