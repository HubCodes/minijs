use lang::ast::Symbol;

pub enum Scope {
    Root,
    Block { parent: Box<Scope>, items: Vec<Symbol> },
    Function { parent: Box<Scope>, args: Vec<Symbol> },
}
