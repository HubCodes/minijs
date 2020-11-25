#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Unop(Unop, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
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
}

#[derive(Debug, PartialEq)]
pub enum Term {
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
pub struct Symbol {
    pub id: i32,
    pub name: String,
}

impl Symbol {
    pub fn new(id: i32, name: &str) -> Symbol {
        Symbol { id, name: name.to_string() }
    }
}
