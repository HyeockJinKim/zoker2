use std::sync::Arc;
use crate::caller::{Context, Func};
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

    pub fn apply(&self, op: Arc<dyn Operation>) -> Context {
        // TODO:
        self.funcs.get(0).unwrap().apply(op)
    }
}
