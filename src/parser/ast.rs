use crate::location::Location;

// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    GlobalStatements(Vec<GlobalStatement>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type GlobalStatement = Located<GlobalStatementType>;
pub type ContractStatement = Located<ContractStatementType>;
pub type Statement = Located<StatementType>;
pub type Expression = Located<ExpressionType>;
pub type Parameter = Located<ParameterType>;

#[derive(Clone, Debug, PartialEq)]
pub enum GlobalStatementType {
    ContractStatement {
        contract_name: String,
        members: Vec<ContractStatement>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContractStatementType {
    FunctionStatement {
        function_name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        statements: Vec<Statement>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    IfStatement {
        condition: Expression,
        if_statements: Vec<Statement>,
        else_statements: Vec<Statement>,
    },
    ForEachStatement {
        iterator: String,
        iterable: Expression,
        statements: Vec<Statement>,
    },
    ReturnStatement {
        return_value: Expression,
    },
    InitializerStatement {
        variable_type: Type,
        variable: String,
        default: Option<Expression>,
    },
    AssignStatement {
        left: String,
        operator: AssignOperator,
        right: Expression,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionType {
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCallExpression {
        function_name: String,
        arguments: Vec<Expression>,
    },
    Number {
        value: u64,
    },
    Identifier {
        value: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParameterType {
    Private {
        variable_type: Type,
        variable: String,
    },
    Public {
        variable_type: Type,
        variable: String,
    }
}

impl ExpressionType {
    pub fn identifier_name(&self) -> Option<String> {
        if let ExpressionType::Identifier { value } = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    NotEq,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignOperator {
    Assign,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    // Static size
    UInt256,
}
