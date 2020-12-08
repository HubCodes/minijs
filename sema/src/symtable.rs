use std::cell::RefCell;
use std::rc::Rc;

use lang::ast::Symbol;

pub enum Scope {
    Root,
    Block { parent: Rc<RefCell<Scope>>, items: Vec<Symbol> },
    Function { parent: Rc<RefCell<Scope>>, args: Vec<Symbol> },
}

pub struct SymbolTable {
    scope: Rc<RefCell<Scope>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { scope: Rc::new(RefCell::new(Scope::Root)) }
    }

    fn enter_block(&self) -> SymbolTable {
        SymbolTable {
            scope: Rc::new(
                RefCell::new(Scope::Block { parent: Rc::clone(&self.scope), items: vec![] })
            )
        }
    }

    fn enter_function(&self) -> SymbolTable {
        SymbolTable {
            scope: Rc::new(
                RefCell::new(Scope::Function { parent: Rc::clone(&self.scope), args: vec![] })
            )
        }
    }

    fn add_def(&mut self, symbol: Symbol) {
        match &mut *(*self.scope).borrow_mut() {
            Scope::Root => (),
            Scope::Block { ref mut items, .. } |
            Scope::Function { args: ref mut items, .. } => {
                items.push(symbol);
            },
        }
    }
}
