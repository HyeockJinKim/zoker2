pub use contract::Contract;
pub use function::Func;
pub use operation::Op;

use std::collections::HashMap;
use crate::generator::Generator;
use crate::variable::Var;

mod contract;
mod function;
pub mod operation;

pub struct Context {
    variables: HashMap<String, Var>,
    stack: Vec<Var>,
}

/// Operation Context (Runtime Context)
impl Context {
    pub(crate) fn new() -> Self {
        Self {
            variables: Default::default(),
            stack: Default::default(),
        }
    }
}

pub trait Caller {
    fn call(&self, ctx: Context, generator: impl Generator);
}