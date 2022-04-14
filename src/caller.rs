pub use contract::Contract;
pub use function::Func;
pub use operation::Op;

mod contract;
mod function;
pub mod operation;

pub trait Caller {
    fn call(&self);
}