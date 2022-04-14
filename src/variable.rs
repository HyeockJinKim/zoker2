use crate::caller::Op;

pub mod uint;
pub mod functor;

pub type Var = Box<dyn Variable>;

pub trait Variable {
    fn name(&self) -> String;
    fn gen(self);
    fn capture(&self) -> Var;
}
