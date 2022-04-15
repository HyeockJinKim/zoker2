use crate::caller::Op;

pub mod uint;
pub mod functor;

pub type Var = Box<dyn Variable>;

pub trait Variable {
    fn name(&self) -> String;
    fn eval(self);
    fn capture(&self) -> Var;
}

pub fn add(a: Var, b: Var) -> Var {
    a
}

pub fn sub(a: Var, b: Var) -> Var {
    a
}

pub fn mul(a: Var, b: Var) -> Var {
    a
}

pub fn div(a: Var, b: Var) -> Var {
    a
}
