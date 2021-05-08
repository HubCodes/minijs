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
        unimplemented!();
    }

    pub fn write(&mut self, inst: IR) {
        self.trans_stack.last_mut().unwrap().push(inst);
    }

    pub fn push_ctxt(&mut self, symbol: Symbol, args: Option<Vec<Symbol>>) {
        self.trans_stack.push(BasicBlock::new(symbol, args));
    }

    pub fn pop_ctxt(&mut self) {
        if let Some(bb) = self.trans_stack.pop() {
            self.bytecode.add(bb);
        }
    }
}
