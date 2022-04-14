/// Zoker source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    // Operator
    // Arithmetic Operator
    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    // Shift operator
    LShift,
    RShift,

    // Assign operator
    Assign,

    // Comparison Operator
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    NotEq,
    // Logical Operator
    And,
    Or,
    // Bit Operator
    BitAnd,
    BitXor,
    BitOr,

    // Type
    // Static size
    UInt256,

    // Keyword
    Function,
    Contract,
    If,
    Else,
    For,
    In,
    Returns,
    Return,
    Private,
    // Mark
    LPar,
    RPar,
    LBrace,
    RBrace,
    Semi,
    Comma,
    // variable
    Num { number: u64 },
    Identifier { name: String },
    Literal { literal: String },
    EOF,
}
