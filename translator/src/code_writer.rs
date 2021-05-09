use lang::ir::{ByteCode, IR, BasicBlock};
use lang::ast::Symbol;

pub struct CodeWriter {
    trans_stack: Vec<BasicBlock>,
    bytecode: ByteCode,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            trans_stack: Vec::new(),
            bytecode: ByteCode::new(),
        }
    }

    pub fn emit_bytecode(&self) -> ByteCode {
        self.bytecode.clone()
    }

    pub fn write(&mut self, inst: IR) {
        self.trans_stack.last_mut().unwrap().push(inst);
    }

    pub fn push_ctxt(&mut self, symbol: Symbol, args: Option<Vec<Symbol>>) {
        let parent = self.trans_stack.last().map(|x| x.symbol.clone());
        self.trans_stack.push(BasicBlock::new(parent, symbol, args));
    }

    pub fn pop_ctxt(&mut self) {
        if let Some(bb) = self.trans_stack.pop() {
            self.bytecode.add(bb);
        }
    }
}
