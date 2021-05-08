use lang::ast::*;
use lang::ir::*;
use crate::code_writer::CodeWriter;

pub struct Translator {
    code_writer: CodeWriter,
}

impl Translator {
    pub fn trans(&mut self, ast: Program) -> ByteCode {
        self.stmt(ast.stmt);
        code_writer.emit_bytecode()
    }

    fn stmt(&mut self, ast: Stmt) {
        match ast {
            Stmt::Block(block) => block.into_iter().for_each(|x| self.stmt(x)),
            Stmt::Expr(expr) => self.expr(expr),
            Stmt::If(cond, then, els) => self.if_stmt(cond, then, els),
            Stmt::VarDef(name, init) => self.var_def(name, init),
        }
    }

    fn expr(&mut self, ast: Expr) {
        unimplemented!();
    }

    fn if_stmt(&mut self, cond: Expr, then: Box<Stmt>, els: Option<Box<Stmt>>) {
        unimplemented!();
    }

    fn var_def(&mut self, name: Symbol, init: Option<Expr>) {
        unimplemented!();
    }
}