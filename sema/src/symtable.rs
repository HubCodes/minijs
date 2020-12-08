use core::mem;

use lang::ast::Symbol;

#[derive(Default)]
pub enum Scope {
    Root,
    Block { parent: Box<Scope>, items: Vec<Symbol> },
    Function { parent: Box<Scope>, args: Vec<Symbol> },
}

pub struct SymbolTable {
    scope: Box<Scope>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { scope: Box::new(Scope::Root) }
    }

    fn enter_scope(&mut self) {
        self.scope = Box::new(Scope::Block { parent: Box::new(*self.scope), items: vec![] });
    }

    fn exit_scope(&mut self) {
        match *self.scope {
            Scope::Root => (),
            Scope::Block { parent, .. } => {
                mem::replace(&mut self.scope, parent);
            },
            Scope::Function { parent, .. } => {
                mem::replace(&mut self.scope, parent);
            },
        }
    }
}
