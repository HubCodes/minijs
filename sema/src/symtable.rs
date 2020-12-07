use lang::ast::Symbol;

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
        let before_scope = *self.scope;
        self.scope = Box::new(Scope::Block { parent: Box::new(before_scope), items: vec![] });
    }

    fn exit_scope(&mut self) {
        let scope = *self.scope;
        match scope {
            Scope::Root => (),
            Scope::Block { parent, .. } => self.scope = parent,
            Scope::Function { parent, .. } => self.scope = parent,
        }
    }
}
