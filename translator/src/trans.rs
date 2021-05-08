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
            Expr::Lambda(sym, args, body) => self.lambda(sym, args, body),
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
        /*
          In this context, Symbol always means "load variable from env", So
          proper IR is Load.
         */
        self.code_writer.write(IR::Load { target: Box::new(sym) });
    }

    fn num(&mut self, num: Num) {
        /*
          Pushes number into VM stack.
         */
        match num {
            Num::Int(value) => self.code_writer.write(IR::PushInt { value }),
            Num::Double(value) =>
                self.code_writer.write(IR::PushDouble { value }),
        }
    }

    fn obj(&mut self, obj: Obj) {
        /*
          Objects are can nested, but the VM resolves nesting recursively :)
         */
        for (key, value) in obj.kv {
            self.expr(value);
            self.code_writer.write(IR::PushString { value: key });
        }
        self.code_writer.write(IR::MakeObject { kv_count: obj.kv.len() });
    }

    fn str(&mut self, str: String) {
        self.code_writer.write(IR::PushString { value: str });
    }

    fn index(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) {
        /*
          Index exprs such as a[i] represents object access via key.
         */
        self.expr(*lhs);
        self.expr(*rhs);
        self.code_writer.write(IR::LoadMemberIndex);
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

    fn lambda(&mut self, sym: Symbol, args: Vec<Expr>, body: Box<Stmt>) {
        unimplemented!();
    }

    fn if_stmt(&mut self, cond: Expr, then: Box<Stmt>, els: Option<Box<Stmt>>) {
        unimplemented!();
    }

    fn var_def(&mut self, name: Symbol, init: Option<Expr>) {
        unimplemented!();
    }
}