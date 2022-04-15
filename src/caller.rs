pub use contract::Contract;
pub use function::Func;
pub use operation::Op;
use crate::generator::Generator;
use crate::variable::Var;

mod contract;
mod function;
pub mod operation;

pub struct Context {
    stack: Vec<Var>,
}

/// Operation Context (Runtime Context)
impl Context {
    pub(crate) fn new() -> Self {
        Self {
            stack: Default::default(),
        }
    }
}

pub trait Caller {
    fn call(&self, ctx: Context, generator: impl Generator);
}