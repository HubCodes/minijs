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
    JumpIfTrue { offset: i32 },
    JumpIfFalse { offset: i32 },
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
    pub label: i32,
    codes: Vec<IR>,
}

impl BasicBlock {
    pub fn new(label: i32) -> BasicBlock {
        BasicBlock { label, codes: Vec::new() }
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

    pub fn add(&mut self, sym: Symbol, bb: BasicBlock) {
        self.code.insert(sym, bb);
    }
}

pub struct BasicBlockGenerator {
    next_label: i32,
}

impl BasicBlockGenerator {
    pub fn new() -> BasicBlockGenerator {
        BasicBlockGenerator { next_label: 0 }
    }

    pub fn next(&mut self) -> BasicBlock {
        self.next_label += 1;
        BasicBlock::new(self.next_label)
    }
}
