use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lang::ast::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeKey(String);

#[derive(Debug)]
pub enum Scope {
    Root,
    Block { parent: Rc<RefCell<Scope>>, items: Vec<Rc<Symbol>> },
    Function { parent: Rc<RefCell<Scope>>, args: Vec<Rc<Symbol>> },
}

#[derive(Debug)]
pub struct SymbolTable {
    scope: Rc<RefCell<Scope>>,
    table: HashMap<Rc<ScopeKey>, Rc<RefCell<Scope>>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            scope: Rc::new(RefCell::new(Scope::Root)), table: HashMap::new()
        }
    }

    pub fn enter_block(&mut self) {
        self.scope = Rc::new(
            RefCell::new(Scope::Block { parent: Rc::clone(&self.scope), items: vec![] })
        );
    }

    pub fn enter_function(&mut self) {
        self.scope = Rc::new(
            RefCell::new(Scope::Function { parent: Rc::clone(&self.scope), args: vec![] })
        );
    }

    pub fn leave(&mut self) {
        let parent_scope = match &mut *(*self.scope).borrow_mut() {
            Scope::Root => panic!("Cannot leave from Scope::Root"),
            Scope::Block { parent, .. } |
            Scope::Function { parent, .. } => Rc::clone(parent),
        };
        self.scope = parent_scope;
    }

    pub fn lookup(&self, name: &String, scope_key: &ScopeKey) -> Option<Rc<Symbol>> {
        let scope = self.table.get(scope_key);
        if let Some(scope) = scope {
            match &*(**scope).borrow() {
                Scope::Root => None,
                Scope::Function { args: items, .. } |
                Scope::Block { items, .. } => {
                    if let Some(symbol) = items.into_iter().find(|item| *item.name == *name) {
                        Some(Rc::clone(symbol))
                    } else {
                        self.lookup(name, scope_key)
                    }
                },
            }
        } else {
            None
        }
    }

    pub fn add_def(&mut self, scope_key: &ScopeKey, symbol: &Symbol) {
        let scope_key = (*scope_key).clone();
        self.table.insert(Rc::new(scope_key), Rc::clone(&self.scope));
        match &mut *(*self.scope).borrow_mut() {
            Scope::Root => panic!("Cannot add definition into Scope::Root"),
            Scope::Block { ref mut items, .. } |
            Scope::Function { args: ref mut items, .. } => {
                items.push(Rc::new(symbol.clone()));
            },
        }
    }
}
