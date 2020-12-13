use lang::ast::*;
use crate::symtable::SymbolTable;

#[derive(Debug)]
pub struct SymbolTableBuilder {
    table: SymbolTable,
}

impl SymbolTableBuilder {
    pub fn new() -> SymbolTableBuilder {
        SymbolTableBuilder { table: SymbolTable::new() }
    }

    pub fn from_ast(&mut self, ast: &Program) -> &SymbolTable {
        self.table.enter_block();
        self.from_stmt(&ast.0);
        self.table.leave();
        &self.table
    }

    fn from_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.from_expr(expr),
            Stmt::VarDef(symbol, expr) => self.from_vardef(symbol, expr),
            Stmt::If(cond, then, els) => self.from_if(cond, then, els),
            Stmt::Block(stmts) => self.from_block(stmts),
        };
    }

    fn from_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Term(term) => self.from_term(term),
            Expr::Binop(_, left, right) => self.from_binop(left, right),
            Expr::Unop(_, operand) => self.from_unop(operand),
            Expr::Call(target, args) => self.from_call(target, args),
            Expr::Lambda(args, body) => self.from_lambda(args, body),
        };
    }

    fn from_vardef(&mut self, symbol: &Symbol, expr: &Option<Expr>) {
        self.table.add_def(symbol);
        if let Some(expr) = expr {
            self.from_expr(expr)
        }
    }

    fn from_if(&mut self, cond: &Expr, then: &Stmt, els: &Option<Box<Stmt>>) {
        self.from_expr(cond);
        self.from_stmt(then);
        if let Some(els) = els {
            self.from_stmt(els);
        }
    }

    fn from_block(&mut self, stmts: &Vec<Stmt>) {
        self.table.enter_block();
        for stmt in stmts {
            self.from_stmt(stmt);
        }
        self.table.leave();
    }

    fn from_term(&mut self, term: &Term) {
        match term {
            Term::Obj(obj) => self.from_obj(obj),
            _ => (),
        }
    }

    fn from_binop(&mut self, left: &Expr, right: &Expr) {
        self.from_expr(left);
        self.from_expr(right);
    }

    fn from_unop(&mut self, operand: &Expr) {
        self.from_expr(operand);
    }

    fn from_call(&mut self, target: &Expr, args: &Vec<Expr>) {
        self.from_expr(target);
        for arg in args {
            self.from_expr(arg);
        }
    }

    fn from_lambda(&mut self, args: &Vec<Expr>, body: &Box<Stmt>) {
        self.table.enter_function();
        for arg in args {
            self.from_expr(arg);
        }
        self.from_stmt(body);
        self.table.leave();
    }

    fn from_obj(&mut self, obj: &Obj) {
        for (_, value) in &obj.kv {
            self.from_expr(value);
        }
    }
}
