use crate::object::Object;
use lang::ast::Symbol;

pub enum Reference {
    Obj(*mut Object),
    Ref(Symbol),
}
