use lang::ir::{ByteCode, IR, BasicBlock, BasicBlockGenerator};

pub struct CodeWriter {
    trans_stack: Vec<BasicBlock>,
    basic_block_generator: BasicBlockGenerator,
    bytecode: ByteCode,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            trans_stack: Vec::new(),
            basic_block_generator: BasicBlockGenerator::new(),
            bytecode: ByteCode::new(),
        }
    }

    pub fn emit_bytecode(&self) -> ByteCode {
        unimplemented!();
    }

    pub fn write(&mut self, inst: IR) {
        self.trans_stack.last_mut().unwrap().push(inst);
    }

    pub fn push_ctxt(&mut self) {
        self.trans_stack.push(self.basic_block_generator.next());
    }

    pub fn pop_ctxt(&mut self) {
        if let Some(bb) = self.trans_stack.pop() {
            //self.bytecode.add()
        }
    }
}
