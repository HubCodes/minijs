use crate::ast::Symbol;
use std::collections::HashMap;

pub enum IR {
    Pop,
    PushBool { value: bool },
    PushInt { value: i32 },
    PushDouble { value: f64 },
    PushString { value: String },
    MakeObject { kv_count: usize },
    Load { target: Box<Symbol> },
    LoadMember { target: Box<Symbol> },
    LoadMemberIndex,
    Call { argc: usize },
    FuncRef { symbol: Box<Symbol> },
    JumpCond { on_true: Box<Symbol>, on_false: Option<Box<Symbol>> },
    Return,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    Neq,
    BitAnd,
    Xor,
    BitOr,
    And,
    Or,
    Typeof,
    Assign,
}

pub struct BasicBlock {
    pub symbol: Symbol,
    pub codes: Vec<IR>,
    pub args: Option<Vec<Symbol>>,
}

impl BasicBlock {
    pub fn new(symbol: Symbol, args: Option<Vec<Symbol>>) -> BasicBlock {
        BasicBlock { symbol, args, codes: Vec::new() }
    }

    pub fn push(&mut self, ir: IR) {
        self.codes.push(ir);
    }
}

pub struct ByteCode {
    code: HashMap<Symbol, BasicBlock>,
}

impl ByteCode {
    pub fn new() -> ByteCode {
        ByteCode { code: HashMap::new() }
    }

    pub fn add(&mut self, bb: BasicBlock) {
        self.code.insert(bb.symbol.clone(), bb);
    }
}
