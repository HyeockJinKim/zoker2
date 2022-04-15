use crate::variable::{Var, Variable};

pub struct Uint {
    name: String,
    privacy: bool, // TODO: privacy 대신 struct를 분리
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

    fn eval(self) {
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

    fn eval(self) {
        todo!()
    }

    fn capture(&self) -> Var {
        Box::new(Self {
            v: self.v,
        })
    }
}
