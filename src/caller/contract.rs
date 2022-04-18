use crate::caller::{Caller, Context, Func};
use crate::generator::Generator;

pub struct Contract {
    name: String,
    funcs: Vec<Func>,
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
    fn call(&self, ctx: Context) -> Context  {
        self.funcs.iter().fold(ctx, |ctx, func| func.call(ctx))
    }
}
