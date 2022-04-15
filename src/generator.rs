mod prover;
mod verifier;

use crate::caller::Caller;
use crate::variable::Var;

pub trait Generator {
    fn generate(&self, var: Var) -> String;
}
