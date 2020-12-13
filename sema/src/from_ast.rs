use lang::ast::*;
use crate::symtable::SymbolTable;
use std::rc::Rc;

#[derive(Debug)]
pub struct SymbolTableBuilder {
    table: SymbolTable,
}

impl SymbolTableBuilder {
    pub fn new() -> SymbolTableBuilder {
        SymbolTableBuilder { table: SymbolTable::new() }
    }

    pub fn from_ast(&mut self, ast: &Program) -> &SymbolTable {
        self.table.enter_block(Rc::clone(&ast.scope_key));
        self.from_stmt(&ast.stmt, &ast.scope_key);
        self.table.leave();
        &self.table
    }

    fn from_stmt(&mut self, stmt: &Stmt, upper_scope_key: &Rc<ScopeKey>) {
        match stmt {
            Stmt::Expr(expr) => self.from_expr(expr, upper_scope_key),
            Stmt::VarDef(symbol, expr) => self.from_vardef(symbol, expr, upper_scope_key),
            Stmt::If(cond, then, els) => self.from_if(cond, then, els, upper_scope_key),
            Stmt::Block(stmts, scope_key) => self.from_block(stmts, scope_key),
        };
    }

    fn from_expr(&mut self, expr: &Expr, upper_scope_key: &Rc<ScopeKey>) {
        match expr {
            Expr::Term(term) => self.from_term(term, upper_scope_key),
            Expr::Binop(_, left, right) => self.from_binop(left, right, upper_scope_key),
            Expr::Unop(_, operand) => self.from_unop(operand, upper_scope_key),
            Expr::Call(target, args) => self.from_call(target, args, upper_scope_key),
            Expr::Lambda(args, body, scope_key) => self.from_lambda(args, body, scope_key),
        };
    }

    fn from_vardef(&mut self, symbol: &Symbol, expr: &Option<Expr>, scope_key: &Rc<ScopeKey>) {
        self.table.add_def(symbol, Rc::clone(scope_key));
        if let Some(expr) = expr {
            self.from_expr(expr, scope_key)
        }
    }

    fn from_if(
        &mut self,
        cond: &Expr,
        then: &Stmt,
        els: &Option<Box<Stmt>>,
        scope_key: &Rc<ScopeKey>
    ) {
        self.from_expr(cond, scope_key);
        self.from_stmt(then, scope_key);
        if let Some(els) = els {
            self.from_stmt(els, scope_key);
        }
    }

    fn from_block(&mut self, stmts: &Vec<Stmt>, scope_key: &Rc<ScopeKey>) {
        self.table.enter_block(Rc::clone(scope_key));
        for stmt in stmts {
            self.from_stmt(stmt, scope_key);
        }
        self.table.leave();
    }

    fn from_term(&mut self, term: &Term, scope_key: &Rc<ScopeKey>) {
        match term {
            Term::Obj(obj) => self.from_obj(obj, scope_key),
            _ => (),
        }
    }

    fn from_binop(&mut self, left: &Expr, right: &Expr, scope_key: &Rc<ScopeKey>) {
        self.from_expr(left, scope_key);
        self.from_expr(right, scope_key);
    }

    fn from_unop(&mut self, operand: &Expr, scope_key: &Rc<ScopeKey>) {
        self.from_expr(operand, scope_key);
    }

    fn from_call(&mut self, target: &Expr, args: &Vec<Expr>, scope_key: &Rc<ScopeKey>) {
        self.from_expr(target, scope_key);
        for arg in args {
            self.from_expr(arg, scope_key);
        }
    }

    fn from_lambda(&mut self, args: &Vec<Expr>, body: &Box<Stmt>, scope_key: &Rc<ScopeKey>) {
        self.table.enter_function(Rc::clone(scope_key));
        for arg in args {
            self.from_expr(arg, scope_key);
        }
        self.from_stmt(body, scope_key);
        self.table.leave();
    }

    fn from_obj(&mut self, obj: &Obj, scope_key: &Rc<ScopeKey>) {
        for (_, value) in &obj.kv {
            self.from_expr(value, scope_key);
        }
    }
}
