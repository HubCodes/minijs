pub enum Expr {
    Term(Term),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Int(i32),
}


