use num_bigint::BigUint;
use crate::location::Location;

// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Program {
    GlobalStatements(Vec<GlobalStatement>),
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type GlobalStatement = Located<GlobalStatementType>;
pub type ContractStatement = Located<ContractStatementType>;
pub type Statement = Located<StatementType>;
pub type Expression = Located<ExpressionType>;
pub type Parameter = Located<ParameterType>;
pub type BinaryExpression = Located<BinaryExpressionType>;

#[derive(Debug, PartialEq)]
pub enum GlobalStatementType {
    ContractStatement {
        contract_name: String,
        members: Vec<ContractStatement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum ContractStatementType {
    FunctionStatement {
        function_name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        statements: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    IfStatement {
        condition: Expression,
        if_statements: Vec<Statement>,
        else_statements: Vec<Statement>,
    },
    ForEachStatement {
        iterator: String,
        iterable: Expression,
        statement: Vec<Statement>,
    },
    ReturnStatement {
        ret: Expression,
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

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    BinaryExpression(BinaryExpression),
    FunctionCallExpression {
        function_name: String,
        arguments: Vec<Expression>,
    },
    Number {
        value: BigUint,
    },
    Identifier {
        value: String,
    },
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum BinaryExpressionType {
    Arithmetic {
        left: Box<Expression>,
        operator: ArithmeticOperator,
        right: Box<Expression>,
    },
    Bit {
        left: Box<Expression>,
        operator: BitOperator,
        right: Box<Expression>,
    },
    Comparison {
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },
    Logical {
        left: Box<Expression>,
        operator: LogicalOperator,
        right: Box<Expression>,
    },
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

#[derive(Debug, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    NotEq,
}

#[derive(Debug, PartialEq)]
pub enum BitOperator {
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
}

#[derive(Debug, PartialEq)]
pub enum AssignOperator {
    Assign,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    // Static size
    UInt256,
}
