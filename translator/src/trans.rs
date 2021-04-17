use lang::ast::*;
use std::rc::Rc;
use sema::symtable::SymbolTable;
use lang::ir::{BasicBlock, IR};

pub struct Translator {
    ast: Box<Program>,
    symbol_table: Box<SymbolTable>,
    trans_stack: Vec<BasicBlock>,
    code_unit_label: i32,
    basic_block_label: i32,
}

impl Translator {
    pub fn new(ast: Box<Program>, symbol_table: Box<SymbolTable>) -> Translator {
        Translator {
            ast,
            symbol_table,
            trans_stack: Vec::new(),
            code_unit_label: 0,
            basic_block_label: 0,
        }
    }

    pub fn translate_intermediate(&mut self) -> Rc<Vec<CodeUnit>> {
        let root_stmt = self.ast.stmt;
        self.trans_stack.push(BasicBlock::new(self.basic_block_label));
        self.basic_block_label += 1;
        self.trans_stmt(&root_stmt);
        Rc::clone(&self.code_units)
    }

    fn trans_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => self.trans_expr(expr),
            Stmt::VarDef(symbol, init) => (),
            Stmt::If(cond, then, els) => (),
            Stmt::Block(stmts, scope_key) => (),
        }
    }

    fn trans_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Term(term) => self.trans_term(term),
            Expr::Binop(op, left, right) => (),
            Expr::Unop(op, operand) => (),
            Expr::Call(target, args) => (),
            Expr::Lambda(args, body, scope_key) => (),
        }
    }

    fn trans_term(&mut self, term: &Term) {
        match term {
            Term::Obj(obj) => (),
            Term::Num(num) => self.trans_num(num),
            Term::Str(str) => (),
            Term::Symbol(symbol) => (),
        }
    }

    fn trans_num(&mut self, num: &Num) {
        match num {
            Num::Int(int) => {
                let ir = IR::PushInt { value: *int };
                self.trans_stack.last_mut().unwrap().codes.push(ir);
            },
            Num::Double(double) => {
                let ir = IR::PushDouble { value: *double };
                self.trans_stack.last_mut().unwrap().codes.push(ir);
            }
        }
    }
}
