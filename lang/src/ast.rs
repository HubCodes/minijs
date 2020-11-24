#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Binop {
    Index,
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
