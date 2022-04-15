use crate::caller::Context;
use crate::generator::Generator;
use crate::variable::Var;

pub type Op = Box<dyn Operation>;
pub trait Operation = FnOnce(Context) -> Context;
pub type BinOp = fn(c1: Var, c2: Var) -> Var;

pub fn push(var: Var) -> Op {
    Box::new(move |mut ctx: Context| {
        ctx.stack.push(var);
        ctx
    })
}

pub fn init(v: Var) -> Op {
    Box::new(move |mut ctx: Context| {
        let name = v.name();
        ctx.variables.insert(name, v);
        ctx
    })
}

pub fn load(var: String) -> Op {
    Box::new(move |mut ctx: Context| {
        let v = ctx.variables.get(var.as_str()).unwrap();
        ctx.stack.push(v.capture());
        ctx
    })
}

pub fn store() -> Op {
    Box::new(move |mut ctx: Context| {
        let v = ctx.stack.pop().unwrap();
        let name = v.name();
        ctx.variables.insert(name, v);
        ctx
    })
}

pub fn bin_op(op: BinOp) -> Op {
    Box::new(move |mut ctx: Context| {
        let c2 = ctx.stack.pop().unwrap();
        let c1 = ctx.stack.pop().unwrap();
        ctx.stack.push(op(c1, c2));
        ctx
    })
}

pub fn condition() -> Op {
    Box::new(move |mut ctx: Context| {
        let c2 = ctx.stack.pop().unwrap();
        let c1 = ctx.stack.pop().unwrap();
        let cond = ctx.stack.pop().unwrap();
        ctx
    })
}

pub fn repeat() -> Op {
    Box::new(move |mut ctx| {
        let c1 = ctx.stack.pop().unwrap();
        let iterable = ctx.stack.pop().unwrap();
        ctx
    })
}

pub fn ret() -> Op {
    Box::new(move |mut ctx: Context| {
        let c1 = ctx.stack.pop().unwrap();
        ctx
    })
}

pub fn call(args_count: usize) -> Op {
    Box::new(move |mut ctx: Context| {
        let mut v = vec![];
        for _ in 0..args_count {
            v.push(ctx.stack.pop().unwrap());
        }
        ctx
    })
}
