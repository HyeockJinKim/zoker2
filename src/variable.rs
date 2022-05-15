use crate::operation::{lazy_private, lazy_public, LazyOp};

pub struct Variable {
    name: String,
    lazy: LazyOp,
}

impl Variable {
    pub fn private(name: String) -> Self {
        Self {
            name: name.clone(),
            lazy: lazy_private(name, 0),
        }
    }

    pub fn public(name: String) -> Self {
        Self {
            name: name.clone(),
            lazy: lazy_public(name, 0),
        }
    }

    pub fn temp(lazy: LazyOp) -> Self {
        Self {
            name: String::new(),
            lazy,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn capture(&self) -> Self {
        Self {
            name: self.name.clone(),
            lazy: self.lazy.clone(),
        }
    }

    pub fn inject(&self) -> LazyOp {
        self.lazy.clone()
    }
}