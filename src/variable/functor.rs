use crate::caller::Op;
use crate::variable::{Var, Variable};

pub struct Functor {
    ops: Vec<Op>,
}

impl Functor {
    pub fn new(ops: Vec<Op>) -> Var {
        Box::new(Self { ops })
    }
}

impl Variable for Functor {
    fn name(&self) -> String {
        todo!()
    }

    fn eval(self) {
        todo!()
    }

    fn capture(&self) -> Var {
        todo!()
    }
}
