use lang::ast::*;
use lang::ir::*;
use crate::code_writer::CodeWriter;

pub struct Translator {
    code_writer: CodeWriter,
}

impl Translator {
    pub fn trans(&mut self, ast: Program) -> ByteCode {
        self.stmt(ast.stmt);
        self.code_writer.emit_bytecode()
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
        match ast {
            Expr::Term(term) => self.term(term),
            Expr::Index(lhs, rhs) => self.index(lhs, rhs),
            Expr::Member(lhs, rhs) => self.member(lhs, rhs),
            Expr::Assign(lhs, rhs) => self.assign(lhs, rhs),
            Expr::Binop(op, lhs, rhs) => self.binop(op, lhs, rhs),
            Expr::Typeof(expr) => self.typeof_expr(expr),
            Expr::Call(func, args) => self.call(func, args),
            Expr::Lambda(args, body) => self.lambda(args, body),
        }
    }

    fn term(&mut self, term: Term) {
        match term {
            Term::Symbol(sym) => self.symbol(sym),
            Term::Num(num) => self.num(num),
            Term::Obj(obj) => self.obj(obj),
            Term::Str(str) => self.str(str),
        }
    }

    fn symbol(&mut self, sym: Symbol) {
        unimplemented!();
    }

    fn num(&mut self, num: Num) {
        unimplemented!();
    }

    fn obj(&mut self, obj: Obj) {
        unimplemented!();
    }

    fn str(&mut self, str: String) {
        unimplemented!();
    }

    fn index(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) {
        unimplemented!();
    }

    fn member(&mut self, lhs: Box<Expr>, rhs: Symbol) {
        unimplemented!();
    }

    fn assign(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) {
        unimplemented!();
    }

    fn binop(&mut self, op: Binop, lhs: Box<Expr>, rhs: Box<Expr>) {
        unimplemented!();
    }

    fn typeof_expr(&mut self, expr: Box<Expr>) {
        unimplemented!();
    }

    fn call(&mut self, func: Box<Expr>, args: Vec<Expr>) {
        unimplemented!();
    }

    fn lambda(&mut self, args: Vec<Expr>, body: Box<Stmt>) {
        unimplemented!();
    }

    fn if_stmt(&mut self, cond: Expr, then: Box<Stmt>, els: Option<Box<Stmt>>) {
        unimplemented!();
    }

    fn var_def(&mut self, name: Symbol, init: Option<Expr>) {
        unimplemented!();
    }
}