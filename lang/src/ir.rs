pub enum IR {
    Pop,
    PushBool { value: bool },
    PushInt { value: i32 },
    PushDouble { value: f64 },
    PushString { value_index: i32 },
    MakeObject { kv_count: i32 },
    Load { target_index: i32 },
    Store { target_index: i32 },
    LoadMember { target_index: i32 },
    StoreMember { target_index: i32 },
    LoadCallable { target_callable_index: i32 },
    Call { argc: i32 },
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
}

pub struct BasicBlock {
    pub label: i32,
    pub codes: Vec<IR>,
}

impl BasicBlock {
    pub fn new(label: i32) -> BasicBlock {
        BasicBlock { label, codes: Vec::new() }
    }
}
