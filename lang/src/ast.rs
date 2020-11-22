pub enum Expr {
    Term(Term),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Num(Num),
}

#[derive(Debug, PartialEq)]
pub enum Num {
    Int(i32),
    Double(f64),
}
