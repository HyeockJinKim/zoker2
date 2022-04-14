use crate::variable::{Var, Variable};

pub struct Uint {
    name: String,
    privacy: bool,
}

impl Uint {
    pub fn new(name: String, privacy: bool) -> Var {
        Box::new(Self {
            name,
            privacy,
        })
    }
}

impl Variable for Uint {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn gen(self) {
        todo!()
    }

    fn capture(&self) -> Var {
        Box::new(Self {
            name: self.name.clone(),
            privacy: self.privacy,
        })
    }
}

pub struct Constant {
    v: u64,
}

impl Constant {
    pub fn new(v: u64) -> Var {
        Box::new(Self {
            v,
        })
    }
}

impl Variable for Constant {
    fn name(&self) -> String {
        "".to_string()
    }

    fn gen(self) {
        todo!()
    }

    fn capture(&self) -> Var {
        Box::new(Self {
            v: self.v,
        })
    }
}
