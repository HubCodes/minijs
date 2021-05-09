use crate::env::Env;
use lang::ir::IR;
use crate::vm_stack::VmStack;
use crate::reference::Reference;
use crate::object::Object;
use allocator::Allocator;
use std::mem::size_of;

pub struct Vm {
    env: Env,
    stack: VmStack,
    allocator: Allocator,
}

impl Vm {
    pub fn eval(&mut self) {
        todo!();
    }

    fn do_eval(&mut self, ir: IR) {
        match ir {
            /*
              Pop Top of stack
             */
            IR::Pop => self.stack.pop(),

            /*
              Push boolean object reference into stack
             */
            IR::PushBool { value } => {
                let mut object = self.allocator.alloc(size_of::<Object>());
                Object::bool(object, value);
                self.stack.push(Reference::Obj(object));
            },

            _ => (),
        }
    }
}
