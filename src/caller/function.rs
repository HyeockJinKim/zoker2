use crate::caller::Op;

pub struct Func {
    name: String,
    ops: Vec<Op>,
}

impl Func {
    pub fn new(name: String, ops: Vec<Op>) -> Self {
        Self{
            name,
            ops,
        }
    }
}