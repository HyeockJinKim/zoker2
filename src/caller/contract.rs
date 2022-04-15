use crate::caller::{Caller, Context, Func};
use crate::generator::Generator;

pub struct Contract {
    name: String,
    funcs: Vec<Func>,
}

impl Contract {
    pub fn new(name: String, funcs: Vec<Func>) -> Self {
        Self {
            name,
            funcs,
        }
    }
}

impl Caller for Contract {
    fn call(&self, ctx: Context, generator: impl Generator) {
        todo!()
    }
}
