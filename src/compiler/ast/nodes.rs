pub struct Program
{
	pub statements: Vec<Node>
}
impl Program
{
	pub fn new() -> Program
	{
		Program { statements: vec![] }
	}
}

pub enum Node
{
	VarDefineNode {
		typ:   Type,
		ident: String,
		expr:  Expr
	},
	VarAssignNode {
		ident: String,
		expr:  Expr
	},
	FunctionNode {
		retType: Type,
		name: String,
		args: Vec<(Type, String)>,
		body: Program
	},
	FuncCallNode {
		name: String,
		args: Vec<Node>
	},
	WhileNode {
		cond: Expr,
		body: Program
	},
	IfNode {
		cond: Expr,
		body: Program
	},
	ImportNode(String),
	URCLBlockNode(String)
}

pub enum Type
{
	Named(String),
	Ptr(Box<Type>),
	Arr(Box<Type>),
	Const(Box<Type>)
}

pub enum Operation
{
	Add,
	Sub,
	Mult,
	Div,
	Mod
}

pub enum Comparison
{
	EQ,
	NEQ,
	LT,
	LTE,
	GT,
	GTE
}

pub enum Expr
{
	Number(i64),
	Ident(String),
	BiOp {
		lhs: Box<Expr>,
		op:  Operation,
		rhs: Box<Expr>
	},
	Comp {
		lhs:  Box<Expr>,
		comp: Comparison,
		rhs:  Box<Expr>
	}
}
