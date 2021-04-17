use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Program {
    pub stmt: Stmt,
    pub scope_key: Rc<ScopeKey>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    VarDef(Symbol, Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Block(Vec<Stmt>, Rc<ScopeKey>),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Unop(Unop, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Expr>, Box<Stmt>, Rc<ScopeKey>),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeKey(pub i32);

impl Symbol {
    pub fn new(id: i32, name: &str) -> Symbol {
        Symbol { id, name: name.to_string() }
    }
}

impl ScopeKey {
    pub fn new(id: i32) -> ScopeKey {
        ScopeKey(id)
    }
}

impl Obj {
    pub fn empty() -> Obj {
        Obj { kv: HashMap::new() }
    }
}
