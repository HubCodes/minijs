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
            IR::Pop => {
                self.stack.pop();
            },

            /*
              Push boolean object reference into stack
             */
            IR::PushBool { value } => {
                let mut object = self.alloc_primitive();
                Object::init_bool(object, value);
                self.push_ref(object);
            },

            /*
              Push int object reference into stack
             */
            IR::PushInt { value } => {
                let mut object = self.alloc_primitive();
                Object::init_int(object, value);
                self.push_ref(object);
            },

            /*
              Push string object reference into stack
             */
            IR::PushString { value } => {
                let mut object = self.alloc_primitive();
                Object::init_string(object, value);
                self.push_ref(object);
            },

            /*
              Pop stack kv_count times, and make object by items
             */
            IR::MakeObject { kv_count } => {
                /*
                  kv_count indicates size of key-value pair
                 */
                let kv_vec: Vec<Option<Reference>> = self.pop_many(kv_count * 2);
                let mut object = self.alloc_primitive();
                let mut kv_pair_vec: Vec<(String, Reference)> = Vec::new();
                for i in kv_vec.len() / 2 {
                    let key: Option<&Reference> = kv_vec.get(i);
                    let key = match key {
                        Some(Reference::Obj(obj)) => obj.get_string(),
                        _ => None,
                    };
                    let value: Option<&Reference> = kv_vec.get(i + 1);
                    if let (Some(key), Some(value)) = (key, value) {
                        kv_pair_vec.push((key.clone(), (*value).clone()));
                    } else {
                        panic!("Key is not string");
                    }
                }
                Object::init_object(object, kv_pair_vec);
                self.push_ref(object);
            },

            _ => (),
        }
    }

    fn alloc_primitive(&mut self) -> *mut Object {
        self.allocator.alloc(size_of::<Object>(), false)
    }

    fn push_ref(&mut self, obj: *mut Object) {
        self.stack.push(Reference::Obj(obj));
    }

    fn pop_many(&mut self, count: usize) -> Vec<Option<Reference>> {
        let mut result = Vec::new();
        for _ in count {
            result.push(self.stack.pop());
        }
        result
    }
}
