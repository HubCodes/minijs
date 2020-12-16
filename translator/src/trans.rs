use lang::ast::*;
use std::rc::Rc;
use sema::symtable::SymbolTable;
use cfg::cfg::CFGNode;

pub struct Translator {
    ast: Box<Program>,
    symbol_table: Box<SymbolTable>,
}

impl Translator {
    pub fn new(ast: Box<Program>, symbol_table: Box<SymbolTable>) -> Translator {
        Translator { ast, symbol_table }
    }

    pub fn translate(&self) -> Box<CFGNode> {
    }
}
