use std::collections::HashMap;
use crate::ast;
use crate::caller::{Contract, Func, Op};
use crate::caller::operation::{bin_op, call, load, push, repeat, ret};
use crate::parser::ast::{BinaryExpressionType, ContractStatementType, ExpressionType, GlobalStatementType, ParameterType, StatementType};
use crate::variable::functor::Functor;
use crate::variable::uint::{Constant, Uint};
use crate::variable::{add, sub, Var};

struct Context {
    ops: Vec<Op>,
}

/// Traverser Context (Compile Context)
impl Context {
    pub(crate) fn new() -> Self {
        Self {
            ops: Default::default(),
        }
    }

    pub(crate) fn sub_context(&self) -> Self {
        /// TODO: compile context에서 type check 정도는 필요해보임
        Self {
            ops: Default::default(),
        }
    }

    pub(crate) fn var(mut self, k: String) -> Self {
        self.ops.push(load(k));
        self
    }

    pub(crate) fn new_var(mut self, v: Var) -> Self {
        self
    }

    pub(crate) fn ops(self) -> Vec<Op> {
        self.ops
    }

    pub(crate) fn with_op(mut self, op: Op) -> Self {
        self.ops.push(op);
        self
    }
}

pub struct ASTTraverser {}

impl ASTTraverser {
    pub fn traverse(ast: ast::Program) -> Vec<Contract> {
        match ast {
            ast::Program::GlobalStatements(v) => v.iter().map(Self::traverse_global_statement).collect()
        }
    }

    fn traverse_global_statement(stmt: &ast::GlobalStatement) -> Contract {
        match &stmt.node {
            GlobalStatementType::ContractStatement { contract_name, members } =>
                Contract::new(contract_name.clone(), members.iter().map(Self::traverse_member).collect())
        }
    }

    fn traverse_member(stmt: &ast::ContractStatement) -> Func {
        match &stmt.node {
            ContractStatementType::FunctionStatement { function_name, parameters, return_type: _return_type, statements } => {
                let ctx = parameters.iter().fold(Context::new(), Self::traverse_parameter);
                let ctx = statements.iter().fold(ctx, Self::traverse_statement);
                Func::new(function_name.clone(), ctx.ops)
            }
        }
    }

    fn traverse_parameter(ctx: Context, param: &ast::Parameter) -> Context {
        match &param.node {
            ParameterType::Private { variable_type: _variable_type, variable } => ctx.new_var(Uint::new(variable.clone(), true)), // name을 key로 pop 해서 var을 넣어야겠다!
            ParameterType::Public { variable_type: _variable_type, variable } => ctx.new_var(Uint::new(variable.clone(), false)),
        }
    }

    fn traverse_statement(ctx: Context, stmt: &ast::Statement) -> Context {
        match &stmt.node {
            StatementType::IfStatement { condition, if_statements, else_statements } => {
                let ctx = Self::traverse_expression(ctx, condition);
                let if_ops = if_statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                let else_ops = else_statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                ctx.with_op(push(Functor::new(if_ops)))
                    .with_op(push(Functor::new(else_ops)))
            }
            StatementType::ForEachStatement { iterator, iterable, statements } => {
                let ctx = Self::traverse_expression(ctx, iterable);
                // TODO: iterator를 받아서 처리
                let ops = statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                ctx.with_op(repeat()) // TODO: iterable을 받아서 처리
                    .with_op(push(Functor::new(ops)))
            }
            StatementType::ReturnStatement { return_value } => Self::traverse_expression(ctx, return_value).with_op(ret()),
            StatementType::InitializerStatement { variable_type: _variable_type, variable, default } => {
                let ctx = ctx.new_var(Uint::new(variable.clone(), false));
                ctx
                // default.map_or(ctx.sub_context(), |expr: &ast::Expression| Self::traverse_expression(ctx, expr)) // TODO: assign 동작
            }
            StatementType::AssignStatement { left, operator, right } => ctx,  // TODO: assign 동작
            StatementType::Expression { expression } => Self::traverse_expression(ctx, expression),
        }
    }

    fn traverse_expression(ctx: Context, expr: &ast::Expression) -> Context {
        match &expr.node {
            ExpressionType::BinaryExpression(bin) => Self::traverse_bin(ctx, bin),
            ExpressionType::FunctionCallExpression { function_name, arguments } => {
                let ctx = ctx.var(function_name.clone());
                arguments.iter().fold(ctx, Self::traverse_expression)
                    .with_op(call(arguments.len()))
            }
            ExpressionType::Number { value } => ctx.with_op(push(Constant::new(value.clone()))),
            ExpressionType::Identifier { value } => ctx.var(value.clone()),
        }
    }

    fn traverse_bin(ctx: Context, expr: &ast::BinaryExpression) -> Context {
        match &expr.node {
            BinaryExpressionType::Arithmetic { left, operator, right } => {

                ctx.with_op(bin_op(add))
            },
            BinaryExpressionType::Bit { left, operator, right } => ctx.with_op(bin_op(sub)),
            BinaryExpressionType::Comparison { left, operator, right } => ctx.with_op(bin_op(add)),
            BinaryExpressionType::Logical { left, operator, right } => ctx.with_op(bin_op(add)),
        }
    }
}