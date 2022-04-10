mod prover;
mod verifier;

use crate::caller::Caller;

pub trait Generator {
    fn generate(&self) -> dyn Caller;
}
