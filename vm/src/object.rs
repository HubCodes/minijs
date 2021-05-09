use lang::ast::Symbol;
use std::collections::HashMap;
use std::ptr::null_mut;

enum Data {
    Obj(HashMap<String, Object>),
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
    prototype: *mut Object,
    data: Data,
}

impl Object {
    pub fn init_bool(obj: *mut Object, value: bool) {
        unsafe {
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::Boolean(value));
        }
    }

    pub fn init_int(obj: *mut Object, value: i32) {
        unsafe {
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::Int(value));
        }
    }

    pub fn init_string(obj: *mut Object, value: String) {
        unsafe {
            (*obj).prototype = null_mut();
            (*obj).data = Data::Primitive(Primitive::String(value));
        }
    }
}
