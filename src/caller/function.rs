use crate::caller::{Caller, Context, Op};
use crate::generator::Generator;

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

impl Caller for Func {
    fn call(&self, ctx: Context, generator: impl Generator) {
        todo!()
    }
}
