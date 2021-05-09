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
                let mut object = self.alloc();
                Object::init_bool(object, value);
                self.push_ref(object);
            },

            /*
              Push int object reference into stack
             */
            IR::PushInt { value } => {
                let mut object = self.alloc();
                Object::init_int(object, value);
                self.push_ref(object);
            },

            /*
              Push string object reference into stack
             */
            IR::PushString { value } => {
                let mut object = self.alloc();
                Object::init_string(object, value);
                self.push_ref(object);
            }

            _ => (),
        }
    }

    fn alloc(&mut self) -> *mut Object {
        self.allocator.alloc(size_of::<Object>())
    }

    fn push_ref(&mut self, obj: *mut Object) {
        self.stack.push(Reference::Obj(obj));
    }
}
