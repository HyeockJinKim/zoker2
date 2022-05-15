use std::sync::Arc;

#[derive(Clone)]
pub struct Runner<T, U> {
    func: Arc<dyn Fn(T) -> U>,
}

impl<T, U> Runner<T, U> {
    pub(crate) fn new(func: Arc<dyn Fn(T) -> U>) -> Self {
        Self {
            func,
        }
    }

    pub(crate) fn run(self, param: T) -> U {
        (self.func)(param)
    }
}

#[derive(Clone)]
pub struct BinRunner<T, U> {
    func: Arc<dyn Fn(T, T) -> U>,
}

impl<T, U> BinRunner<T, U> {
    pub fn new(func: Arc<dyn Fn(T, T) -> U>) -> Self {
        Self {
            func,
        }
    }

    pub fn run(self, param1: T, param2: T) -> U {
        (self.func)(param1, param2)
    }
}
