use crate::caller::Context;
use crate::generator::Generator;
use crate::variable::Var;

pub(crate) type Op = Box<dyn Operation>;
pub(crate) trait Operation = FnOnce(Context) -> Context;
pub(crate) type BinOp = fn(c1: Var, c2: Var) -> Var;

pub(crate) fn push(var: Var) -> Op {
    Box::new(move |mut ctx: Context| {
        ctx.stack.push(var);
        ctx
    })
}

pub(crate) fn init(v: Var) -> Op {
    Box::new(move |mut ctx: Context| {
        // TODO: 이거 아님
        // let name = v.name();
        // ctx.variables.insert(name, v);
        ctx
    })
}

pub(crate) fn load(var: String) -> Op {
    Box::new(move |mut ctx: Context| {
        let v = ctx.variables.get(var.as_str()).unwrap();
        ctx.stack.push(v.capture());
        ctx
    })
}

pub(crate) fn store(k: String) -> Op {
    Box::new(move |mut ctx: Context| {
        let v = ctx.stack.pop().unwrap();
        ctx.variables.insert(k, v);
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

pub(crate) fn condition() -> Op {
    Box::new(move |mut ctx: Context| {
        let c2 = ctx.stack.pop().unwrap();
        let c1 = ctx.stack.pop().unwrap();
        let cond = ctx.stack.pop().unwrap();
        ctx
    })
}

pub(crate) fn repeat() -> Op {
    Box::new(move |mut ctx| {
        let c1 = ctx.stack.pop().unwrap();
        let iterable = ctx.stack.pop().unwrap();
        ctx
    })
}

pub(crate) fn ret() -> Op {
    Box::new(move |mut ctx: Context| {
        // TODO: 현재는 아무 작업도 하지 않음
        // let c1 = ctx.stack.pop().unwrap();
        ctx
    })
}

pub(crate) fn call(args_count: usize) -> Op {
    Box::new(move |mut ctx: Context| {
        let mut v = vec![];
        for _ in 0..args_count {
            v.push(ctx.stack.pop().unwrap());
        }
        ctx
    })
}
