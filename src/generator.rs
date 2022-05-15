use crate::caller::Caller;
use crate::variable::Variable;

pub trait Generator {
    fn generate(&self, var: Variable) -> String;
}
