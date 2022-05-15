use std::sync::Arc;
use crate::caller::Caller;
use crate::operation::{Context, LazyOp, Operation};

pub struct Func {
    name: String,
    ret: LazyOp,
}

impl Func {
    pub(crate) fn new(name: String, ret: LazyOp) -> Self {
        Self{
            name,
            ret,
        }
    }
}

impl Caller for Func {
    fn call(&self, op: Arc<dyn Operation>) -> Context {
        self.ret.clone().run(op)
    }
}
