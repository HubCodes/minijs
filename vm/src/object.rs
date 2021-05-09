use lang::ast::Symbol;
use std::collections::HashMap;
use std::ptr::null_mut;

enum Data {
    Obj { data: HashMap<String, Object> },
    Primitive(Primitive),
}

enum Primitive {
    Int(i32),
    Double(f64),
    Boolean(bool),
}

pub struct Object {
    prototype: *mut Object,
    data: Data,
}

impl Object {
    pub fn init_bool(obj: *mut Object, value: bool) {
        obj.prototype = null_mut();
        obj.data = Data::Primitive(Primitive::Boolean(value));
    }
}
