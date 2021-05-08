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
            Stmt::Block(sym, block) => self.block(sym, block),
            Stmt::Expr(expr) => self.expr(expr),
            Stmt::If(cond, then, els) => self.if_stmt(cond, then, els),
            Stmt::VarDef(name, init) => self.var_def(name, init),
        }
    }

    fn block(&mut self, sym: Symbol, block: Vec<Stmt>) {
        self.code_writer.push_ctxt(sym, None);
        block.into_iter().for_each(|x| self.stmt(x));
        self.code_writer.pop_ctxt();
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
        let kv_count = obj.kv.len();
        for (key, value) in obj.kv {
            self.expr(value);
            self.code_writer.write(IR::PushString { value: key });
        }
        self.code_writer.write(IR::MakeObject { kv_count });
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
        /*
          Member exprs such as a.i represents object access via key which is
          compatible with language symbol.
         */
        self.expr(*lhs);
        self.code_writer.write(IR::LoadMember { target: Box::new(rhs) });
    }

    fn assign(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) {
        /*
          Assign Top of stack into Top-1 of stack.
         */
        self.expr(*lhs);
        self.expr(*rhs);
        self.code_writer.write(IR::Assign);
    }

    fn binop(&mut self, op: Binop, lhs: Box<Expr>, rhs: Box<Expr>) {
        let inst = match op {
            Binop::Mul => IR::Mul,
            Binop::Div => IR::Div,
            Binop::Mod => IR::Mod,
            Binop::Add => IR::Add,
            Binop::Sub => IR::Sub,
            Binop::Shl => IR::Shl,
            Binop::Shr => IR::Shr,
            Binop::Lt => IR::Lt,
            Binop::Gt => IR::Gt,
            Binop::Lte => IR::Lte,
            Binop::Gte => IR::Gte,
            Binop::Eq => IR::Eq,
            Binop::Neq => IR::Neq,
            Binop::BitAnd => IR::BitAnd,
            Binop::Xor => IR::Xor,
            Binop::BitOr => IR::BitOr,
            Binop::And => IR::And,
            Binop::Or => IR::Or,
        };

        self.expr(*lhs);
        self.expr(*rhs);
        self.code_writer.write(inst);
    }

    fn typeof_expr(&mut self, expr: Box<Expr>) {
        self.expr(*expr);
        self.code_writer.write(IR::Typeof);
    }

    fn call(&mut self, func: Box<Expr>, args: Vec<Expr>) {
        self.expr(*func);

        let argc = args.len();
        args.into_iter().for_each(|x| self.expr(x));
        self.code_writer.write(IR::Call { argc });
    }

    fn lambda(&mut self, sym: Symbol, args: Vec<Symbol>, body: Box<Stmt>) {
        self.code_writer.push_ctxt(sym.clone(), Some(args));
        self.stmt(*body);
        self.code_writer.pop_ctxt();
        self.code_writer.write(IR::FuncRef { symbol: sym });
    }

    fn if_stmt(&mut self, cond: Expr, then: Box<Stmt>, els: Option<Box<Stmt>>) {
        unimplemented!();
    }

    fn var_def(&mut self, name: Symbol, init: Option<Expr>) {
        unimplemented!();
    }
}