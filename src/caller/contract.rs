use std::sync::Arc;
use crate::caller::{Caller, Context, Func};
use crate::operation::Operation;

pub struct Contract {
    name: String,
    pub funcs: Vec<Func>,
}

impl Contract {
    pub(crate) fn new(name: String, funcs: Vec<Func>) -> Self {
        Self {
            name,
            funcs,
        }
    }
}

impl Caller for Contract {
    fn call(&self, op: Arc<dyn Operation>) -> Context {
        // TODO:
        self.funcs[0].call(op)
    }
}
