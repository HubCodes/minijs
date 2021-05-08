use lang::ir::{ByteCode, IR, BasicBlock};

pub struct CodeWriter {
    trans_stack: Vec<BasicBlock>,
}

impl CodeWriter {
    pub fn emit_bytecode(&self) -> ByteCode {
        unimplemented!();
    }

    pub fn write(&mut self, inst: IR) {
        self.trans_stack.last().unwrap().push(inst);
    }
}
