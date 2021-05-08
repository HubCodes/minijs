use crate::ast::Symbol;
use std::collections::HashMap;

pub enum IR {
    Pop,
    PushBool { value: bool },
    PushInt { value: i32 },
    PushDouble { value: f64 },
    AllocString { value: Box<String> },
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
    pub code: HashMap<Symbol, BasicBlock>,
}
