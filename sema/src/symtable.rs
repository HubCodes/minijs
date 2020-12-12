use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lang::ast::Symbol;

#[derive(PartialEq, Eq, Hash)]
pub struct ScopeKey(String);

pub enum Scope {
    Root,
    Block { parent: Rc<RefCell<Scope>>, items: Vec<Rc<Symbol>> },
    Function { parent: Rc<RefCell<Scope>>, args: Vec<Rc<Symbol>> },
}

pub struct SymbolTable {
    scope: Rc<RefCell<Scope>>,
    table: HashMap<Rc<ScopeKey>, Rc<RefCell<Scope>>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            scope: Rc::new(RefCell::new(Scope::Root)), table: HashMap::new()
        }
    }

    fn enter_block(&mut self) {
        self.scope = Rc::new(
            RefCell::new(Scope::Block { parent: Rc::clone(&self.scope), items: vec![] })
        );
    }

    fn enter_function(&mut self) {
        self.scope = Rc::new(
            RefCell::new(Scope::Function { parent: Rc::clone(&self.scope), args: vec![] })
        );
    }

    fn lookup(&self, symbol: &Symbol, scope_key: &ScopeKey) -> Option<Rc<Symbol>> {
        let scope = self.table.get(scope_key);
        if let Some(scope) = scope {
            match &*(**scope).borrow() {
                Scope::Root => None,
                Scope::Function { args: items, .. } |
                Scope::Block { items, .. } => {
                    if let Some(symbol) = items.into_iter().find(|item| *item.name == symbol.name) {
                        Some(Rc::clone(symbol))
                    } else {
                        self.lookup(symbol, scope_key)
                    }
                },
            }
        } else {
            None
        }
    }

    fn add_def(&mut self, symbol: Symbol) -> Rc<ScopeKey> {
        let key = symbol.clone();
        let scope_key = Rc::new(ScopeKey(key.name));
        self.table.insert(Rc::clone(&scope_key), Rc::clone(&self.scope));
        match &mut *(*self.scope).borrow_mut() {
            Scope::Root => panic!("Cannot add definition into Scope::Root"),
            Scope::Block { ref mut items, .. } |
            Scope::Function { args: ref mut items, .. } => {
                items.push(Rc::new(symbol));
                Rc::clone(&scope_key)
            },
        }
    }
}
