use lang::ast::Symbol;
use std::collections::HashMap;
use std::ptr::null_mut;
use std::iter::FromIterator;
use crate::reference::Reference;

enum Data {
    Obj(HashMap<String, Reference>),
    Ref(Symbol),
    Primitive(Primitive),
}

enum Primitive {
    Int(i32),
    Double(f64),
    Boolean(bool),
    String(String),
}

pub struct Object {
    /*
      _marker is hint for garbage collector that didn't know about memory state.
      It is redundant if use more space to manage used space, but IMHO
      this can behave as more simple solution.
     */
    pub _marker: u8,
    prototype: *mut Object,
    data: Data,
}

impl Object {
    pub fn init_bool(obj: *mut Object, value: bool) {
        unsafe {
            (*obj)._marker = 1;
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::Boolean(value));
        }
    }

    pub fn init_int(obj: *mut Object, value: i32) {
        unsafe {
            (*obj)._marker = 1;
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::Int(value));
        }
    }

    pub fn init_string(obj: *mut Object, value: String) {
        unsafe {
            (*obj)._marker = 1;
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::String(value));
        }
    }

    pub fn init_object(obj: *mut Object, kv_vec: Vec<(String, Reference)>) {
        unsafe {
            (*obj)._marker = 1;
            (*obj).prototype = null_mut();  // TODO: link prototype object
            (*obj).data = Data::Obj(HashMap::from_iter(kv_vec.into_iter()));
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self.data {
            Data::Primitive(&Primitive::String(str)) => Some(&str),
            _ => None,
        }
    }
}
