use crate::caller::Func;

pub struct Contract {
    name: String,
    funcs: Vec<Func>,
}

impl Contract {
    pub fn new(name: String, funcs: Vec<Func>) -> Self {
        Self {
            name,
            funcs,
        }
    }
}