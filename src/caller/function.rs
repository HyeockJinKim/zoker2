use std::sync::Arc;
use crate::operation::{Context, LazyOp, Operation};

pub struct Func {
    name: String,
    ret: LazyOp,
}

impl Func {
    pub(crate) fn new(name: String, ret: LazyOp) -> Self {
        Self {
            name,
            ret,
        }
    }

    pub fn apply(&self, op: Arc<dyn Operation>) -> Context {
        self.ret.clone().run(op)
    }
}
