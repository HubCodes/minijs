use core::mem;
use std::rc::Rc;

use lang::ast::Symbol;

#[derive(Default)]
pub enum Scope {
    Root,
    Block { parent: Rc<Scope>, items: Vec<Symbol> },
    Function { parent: Rc<Scope>, args: Vec<Symbol> },
}

pub struct SymbolTable {
    scope: Rc<Scope>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { scope: Rc::new(Scope::Root) }
    }

    fn enter_block(&self) -> SymbolTable {
        SymbolTable {
            scope: Rc::new(Scope::Block { parent: Rc::clone(&self.scope), items: vec![] })
        }
    }

    fn enter_function(&self) -> SymbolTable {
        SymbolTable {
            scope: Rc::new(Scope::Function { parent: Rc::clone(&self.scope), args: vec![] })
        }
    }
}
