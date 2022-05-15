use std::sync::Arc;
use crate::utils::Runner;

mod calculation;

pub trait Operation {
    fn constant(&self, v: u64) -> Context;
    fn public(&self, name: String, v: u64) -> Context;
    fn private(&self, name: String, v: u64) -> Context;

    fn add(&self, a: Context, b: Context) -> Context;
    fn sub(&self, a: Context, b: Context) -> Context;
    fn mul(&self, a: Context, b: Context) -> Context;
    fn div(&self, a: Context, b: Context) -> Context;
    // TODO: 이후 구현
    // fn div_mod(a: impl Context, b: impl Context) -> dyn Context;
    // fn and(a: impl Context, b: impl Context) -> dyn Context;
    // fn or(a: impl Context, b: impl Context) -> dyn Context;
    // fn bit_and(a: impl Context, b: impl Context) -> dyn Context;
    // fn bit_or(a: impl Context, b: impl Context) -> dyn Context;
    // fn bit_xor(a: impl Context, b: impl Context) -> dyn Context;
    // fn eq(a: impl Context, b: impl Context) -> dyn Context;
    // fn not_eq(a: impl Context, b: impl Context) -> dyn Context;
    // fn gt(a: impl Context, b: impl Context) -> dyn Context;
    // fn ge(a: impl Context, b: impl Context) -> dyn Context;
    // fn lt(a: impl Context, b: impl Context) -> dyn Context;
    // fn le(a: impl Context, b: impl Context) -> dyn Context;
    // fn lshift(a: impl Context, b: impl Context) -> dyn Context;
    // fn rshift(a: impl Context, b: impl Context) -> dyn Context;
}

/// TODO: Fn 을 Clone 또는 Copy하기 위한 wrapper를 둬야할 것
pub(crate) type LazyOp = Runner<Arc<dyn Operation>, Context>;

pub fn lazy_constant(a: u64) -> LazyOp {
    Runner::new(Arc::new(move |op| op.constant(a)))
}

pub fn lazy_private(name: String, a: u64) -> LazyOp {
    Runner::new(Arc::new(move |op| op.private(name.clone(), a)))
}

pub fn lazy_public(name: String, a: u64) -> LazyOp {
    Runner::new(Arc::new(move |op| op.public(name.clone(), a)))
}

pub fn lazy_add(a: LazyOp, b: LazyOp) -> LazyOp {
    Runner::new(Arc::new(move |op| {
        let a_res = a.clone().run(op.clone());
        let b_res = b.clone().run(op.clone());
        op.add(a_res, b_res)
    }))
}

pub fn lazy_sub(a: LazyOp, b: LazyOp) -> LazyOp {
    Runner::new(Arc::new(move |op| {
        let a_res = a.clone().run(op.clone());
        let b_res = b.clone().run(op.clone());
        op.sub(a_res, b_res)
    }))
}

pub fn lazy_mul(a: LazyOp, b: LazyOp) -> LazyOp {
    Runner::new(Arc::new(move |op| {
        let a_res = a.clone().run(op.clone());
        let b_res = b.clone().run(op.clone());
        op.mul(a_res, b_res)
    }))
}

pub fn lazy_div(a: LazyOp, b: LazyOp) -> LazyOp {
    Runner::new(Arc::new(move |op| {
        let a_res = a.clone().run(op.clone());
        let b_res = b.clone().run(op.clone());
        op.div(a_res, b_res)
    }))
}

#[derive(Clone)]
pub struct Context {
    name: Option<String>,
    v: Vec<u32>,
}

/// Variable Context (lazy 연산을 위한 context)
impl Context {
    pub fn var(name: String, v: u32) -> Self {
        Self {
            name: Some(name),
            v: vec![v],
        }
    }

    pub fn temp(v: u32) -> Self {
        Self {
            name: None,
            v: vec![v],
        }
    }

    pub fn capture(&self) -> Self {
        Self {
            name: self.name.clone(),
            v: self.v.clone(),
        }
    }
}
