use lang::ir::BasicBlock;

pub struct CFGNode {
    bb: BasicBlock,
    transitions: Vec<CFGNode>,
}
