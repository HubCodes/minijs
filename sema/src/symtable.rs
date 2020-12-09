use std::cell::RefCell;
use std::rc::Rc;

use lang::ast::Symbol;

pub enum Scope {
    Root,
    Block { parent: Rc<RefCell<Scope>>, items: Vec<Rc<Symbol>> },
    Function { parent: Rc<RefCell<Scope>>, args: Vec<Rc<Symbol>> },
}

pub struct SymbolTable {
    scope: Rc<RefCell<Scope>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable { scope: Rc::new(RefCell::new(Scope::Root)) }
    }

    fn enter_block(&mut self) {
        self.scope = Rc::new(
            RefCell::new(Scope::Block { parent: Rc::clone(&self.scope), items: vec![] })
        );
    }

    fn enter_function(&self) -> SymbolTable {
        SymbolTable {
            scope: Rc::new(
                RefCell::new(Scope::Function { parent: Rc::clone(&self.scope), args: vec![] })
            )
        }
    }

    fn add_def(&mut self, symbol: Rc<Symbol>) {
        match &mut *(*self.scope).borrow_mut() {
            Scope::Root => (),
            Scope::Block { ref mut items, .. } |
            Scope::Function { args: ref mut items, .. } => {
                items.push(Rc::clone(&symbol));
            },
        }
    }
}
