use lang::ast::Symbol;
use std::collections::HashMap;

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
