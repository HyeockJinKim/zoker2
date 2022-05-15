use crate::operation::{Context, Operation};

pub struct COperation {}

impl Operation for COperation {
    fn constant(&self, v: u64) -> Context {
        Context::temp(v as u32)
    }

    fn public(&self, name: String, v: u64) -> Context {
        // TODO:
        Context::var(name, v as u32)
    }

    fn private(&self, name: String, v: u64) -> Context {
        // TODO:
        Context::var(name, v as u32)
    }

    fn add(&self, a: Context, b: Context) -> Context {
        Context::temp(a.v[0] + b.v[0])
    }

    fn sub(&self, a: Context, b: Context) -> Context {
        Context::temp(a.v[0] - b.v[0])
    }

    fn mul(&self, a: Context, b: Context) -> Context {
        Context::temp(a.v[0] * b.v[0])
    }

    fn div(&self, a: Context, b: Context) -> Context {
        Context::temp(a.v[0] / b.v[0])
    }
}
