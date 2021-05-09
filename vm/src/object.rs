use lang::ast::Symbol;
use std::collections::HashMap;

enum Data {
    Ref(*Symbol),
    Ptr(*Object),
}

pub struct Object {
    prototype: *Object,
    data: HashMap<String, Data>,
}
