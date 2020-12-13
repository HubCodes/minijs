use std::collections::HashMap;

pub struct Program(pub Stmt);

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    VarDef(Symbol, Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Block(Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Unop(Unop, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Expr>, Box<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Unop {
    Typeof,
}

#[derive(Debug, PartialEq)]
pub enum Binop {
    Index,
    Member,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Shl,
    Shr,
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    Neq,
    BitAnd,
    Xor,
    BitOr,
    And,
    Or,
    Assign,
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Obj(Obj),
    Num(Num),
    Str(String),
    Symbol(Symbol),
}

#[derive(Debug, PartialEq)]
pub enum Num {
    Int(i32),
    Double(f64),
}

#[derive(Debug, PartialEq)]
pub struct Obj {
    pub kv: HashMap<String, Expr>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol {
    pub id: i32,
    pub name: String,
}

impl Symbol {
    pub fn new(id: i32, name: &str) -> Symbol {
        Symbol { id, name: name.to_string() }
    }
}

impl Obj {
    pub fn empty() -> Obj {
        Obj { kv: HashMap::new() }
    }
}
