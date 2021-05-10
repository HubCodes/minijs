use crate::reference::Reference;

pub struct VmStack {
    stack: Vec<Reference>,
}

impl VmStack {
    pub fn pop(&mut self) -> Option<Reference> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: Reference) {
        self.stack.push(item);
    }
}
