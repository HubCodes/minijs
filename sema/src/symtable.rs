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

    fn enter_block(mut self) {
        let before_scope = self.scope;
        self.scope = Box::new(Scope::Block { parent: before_scope, items: vec![] });
    }

    fn get_scope(&mut self) -> Box<Scope> {
        scope
    }

    fn exit_block(mut self) {
        match *self.scope {
            Scope::Root => (),
            Scope::Block { parent, .. } => self.scope = parent,
            Scope::Function { parent, .. } => self.scope = parent,
        }
    }
}
