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
