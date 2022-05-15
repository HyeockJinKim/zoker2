use std::collections::HashMap;
use crate::ast;
use crate::caller::{Contract, Func};
use crate::operation::{lazy_add, lazy_constant, lazy_div, lazy_mul, lazy_sub, lazy_private, lazy_public, LazyOp};
use crate::parser::ast::{BinaryOperator, ContractStatementType, ExpressionType, GlobalStatementType, ParameterType, StatementType};
use crate::variable::Variable;

struct Context {
    variables: HashMap<String, Variable>,
    stack: Vec<Variable>,
}

/// Traverser Context (Compile Context)
impl Context {
    fn new() -> Self {
        Self {
            variables: Default::default(),
            stack: vec![],
        }
    }

    fn sub_context(&self) -> Self {
        /// TODO: compile context에서 type check 정도는 필요해보임
        let mut variables: HashMap<String, Variable> = Default::default();
        self.variables.iter().for_each(|(k, v)| {
            variables.insert(k.clone(), v.capture());
        });
        Self {
            variables,
            stack: vec![],
        }
    }

    fn push(mut self, v: Variable) -> Self {
        self.stack.push(v);
        self
    }

    fn load(mut self, k: String) -> Self {
        let v = self.variables.get(k.as_str()).unwrap();
        self.stack.push(v.capture());
        self
    }

    fn store(mut self, k: String) -> Self {
        let v = self.stack.pop().unwrap();
        self.variables.insert(k.clone(), v);
        let v = self.variables.get(k.as_str()).unwrap();
        self.stack.push(v.capture());
        self
    }

    fn bin_op(mut self, bin_op: impl Fn(LazyOp, LazyOp) -> LazyOp) -> Self {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(Variable::temp(bin_op(a.inject(), b.inject())));
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
                let mut ctx = statements.iter().fold(ctx, Self::traverse_statement);
                Func::new(function_name.clone(), ctx.stack.pop().unwrap().inject())
            }
        }
    }

    fn traverse_parameter(ctx: Context, param: &ast::Parameter) -> Context {
        match &param.node {
            ParameterType::Private { variable_type: _variable_type, variable }
            => ctx.push(Variable::private(variable.clone()))
                .store(variable.clone()), // TODO: 이후 타입도 같이 넘겨야 함
            ParameterType::Public { variable_type: _variable_type, variable }
            => ctx.push(Variable::public(variable.clone()))
                .store(variable.clone()),
        }
    }

    fn traverse_statement(ctx: Context, stmt: &ast::Statement) -> Context {
        match &stmt.node {
            StatementType::IfStatement { condition, if_statements, else_statements } => {
                let ctx = Self::traverse_expression(ctx, condition);
                // TODO: condition을 적용할 연산을 정의해야함 (assign에서 11111 을 & 하고 더할 값)
                // let if_ops = if_statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                // let else_ops = else_statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                // ctx.with_op(push(Functor::new(if_ops)))
                //     .with_op(push(Functor::new(else_ops)))
                ctx
            }
            StatementType::ForEachStatement { iterator, iterable, statements } => {
                // TODO: foreach는 vector를 type 추가한 후에 구현 가능
                let ctx = Self::traverse_expression(ctx, iterable);
                // TODO: iterator를 받아서 처리
                // let ops = statements.iter().fold(ctx.sub_context(), Self::traverse_statement).ops();
                // ctx.with_op(repeat()) // TODO: iterable을 받아서 처리
                //     .with_op(push(Functor::new(ops)))
                ctx
            }
            StatementType::ReturnStatement { return_value } => Self::traverse_expression(ctx, return_value),
            StatementType::InitializerStatement { variable_type: _variable_type, variable, default } => {
                let ctx = ctx.push(Variable::public(variable.clone())).store(variable.clone());
                match default {
                    Some(expr) => Self::traverse_expression(ctx, expr),
                    None => ctx,
                }
            }
            StatementType::AssignStatement { left, operator: _, right } => {
                Self::traverse_expression(ctx, right).store(left.clone())
            }
            StatementType::Expression { expression } => Self::traverse_expression(ctx, expression),
        }
    }

    fn traverse_expression(ctx: Context, expr: &ast::Expression) -> Context {
        match &expr.node {
            ExpressionType::BinaryExpression {
                left, operator, right
            } => {
                let ctx = Self::traverse_expression(ctx, left);
                let ctx = Self::traverse_expression(ctx, right);
                Self::traverse_operator(ctx, operator)
            }
            ExpressionType::FunctionCallExpression { function_name, arguments } => {
                arguments.iter().fold(ctx, Self::traverse_expression)
                // .with_op(call(arguments.len()))
            }
            ExpressionType::Number { value } => ctx.push(Variable::temp(lazy_constant(value.clone()))),
            ExpressionType::Identifier { value } => ctx.push(Variable::public(value.clone())),
        }
    }

    fn traverse_operator(ctx: Context, op: &ast::BinaryOperator) -> Context {
        match op {
            BinaryOperator::Add => ctx.bin_op(lazy_add),
            BinaryOperator::Sub => ctx.bin_op(lazy_sub),
            BinaryOperator::Mul => ctx.bin_op(lazy_mul),
            BinaryOperator::Div => ctx.bin_op(lazy_div),
            // TODO: 나중에 구현
            // BinaryOperator::Mod => ctx.with_op(bin_op(add)),
            // BinaryOperator::And => ctx.with_op(bin_op(add)),
            // BinaryOperator::Or => ctx.with_op(bin_op(add)),
            // BinaryOperator::Lt => ctx.with_op(bin_op(add)),
            // BinaryOperator::Le => ctx.with_op(bin_op(add)),
            // BinaryOperator::Gt => ctx.with_op(bin_op(add)),
            // BinaryOperator::Ge => ctx.with_op(bin_op(add)),
            // BinaryOperator::Eq => ctx.with_op(bin_op(add)),
            // BinaryOperator::NotEq => ctx.with_op(bin_op(add)),
            // BinaryOperator::BitAnd => ctx.with_op(bin_op(add)),
            // BinaryOperator::BitOr => ctx.with_op(bin_op(add)),
            // BinaryOperator::BitXor => ctx.with_op(bin_op(add)),
            // BinaryOperator::LShift => ctx.with_op(bin_op(add)),
            // BinaryOperator::RShift => ctx.with_op(bin_op(add)),
            _ => ctx,
        }
    }
}