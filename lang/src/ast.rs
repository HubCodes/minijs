#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Unop(Unop, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Unop {
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
