use std::collections::HashMap;
use crate::ast;
use crate::caller::{Contract, Func, Op};
use crate::parser::ast::{BinaryExpressionType, ContractStatementType, ExpressionType, GlobalStatementType, ParameterType, StatementType};

struct Var {
    id: VarID,
    typ: VarType,
}

type VarID = usize;
enum VarType {
    Private,
    Public,
}

struct Context {
    variables: HashMap<String, Var>,
    ops: Vec<Op>,
    last_var: Option<Var>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
            ops: Default::default(),
            last_var: None,
        }
    }

    pub fn with_var(mut self, var: String) -> Self {
        if !self.variables.contains_key(var.as_str()) {
            let variable = Var{
                id: self.variables.len(),
                typ: VarType::Public,
            };
            self.variables.insert(var, variable);
        }
        self
    }

    pub fn with_private_var(mut self, var: String) -> Self {
        if !self.variables.contains_key(var.as_str()) {
            let variable = Var{
                id: self.variables.len(),
                typ: VarType::Private,
            };
            self.variables.insert(var, variable);
        }
        self
    }

    pub fn with_op(mut self, op: Op) -> Self {
        self.ops.push(op);
        self
    }
}

pub struct GeneratorTraverser {}

impl GeneratorTraverser {
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
            ParameterType::Private { variable_type: _variable_type, variable } => ctx.with_private_var(variable.clone()),
            ParameterType::Public { variable_type: _variable_type, variable } => ctx.with_var(variable.clone()),
        }
    }

    fn traverse_statement(ctx: Context, stmt: &ast::Statement) -> Context {
        match &stmt.node {
            StatementType::IfStatement { condition, if_statements, else_statements } => ctx.with_op(Op::if_op()),
            StatementType::ForEachStatement { iterator, iterable, statement } => ctx.with_op(Op::for_op()),
            StatementType::ReturnStatement { ret } => ctx.with_op(Op::return_op()),
            StatementType::InitializerStatement { variable_type: _variable_type, variable, default } => {
                let ctx = ctx.with_var(variable.clone());
                ctx
            }
            StatementType::AssignStatement { left, operator, right } => ctx,
            StatementType::Expression { expression } => ctx,
        }
    }

    fn traverse_expression(ctx: Context, expr: &ast::Expression) -> Context {
        match &expr.node {
            ExpressionType::BinaryExpression(bin) => ctx,
            ExpressionType::FunctionCallExpression { function_name, arguments } => ctx,
            ExpressionType::Number { value } => ctx,
            ExpressionType::Identifier { value } => ctx,
        }
    }

    fn traverse_bin(ctx: Context, expr: &ast::BinaryExpression) -> Context {
        match &expr.node {
            BinaryExpressionType::Arithmetic { left, operator, right } => ctx.with_op(Op::bin_op()),
            BinaryExpressionType::Bit { left, operator, right } => ctx.with_op(Op::bin_op()),
            BinaryExpressionType::Comparison { left, operator, right } => ctx.with_op(Op::bin_op()),
            BinaryExpressionType::Logical { left, operator, right } => ctx.with_op(Op::bin_op()),
        }
    }
}