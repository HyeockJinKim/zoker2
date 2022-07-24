use lalrpop_util::ParseError as LalrpopError;
use crate::location::Location;
use crate::parser::Tok;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct ZokError {
    pub error: ZokErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum ZokErrorType {
    /// Parser Error Type
    /// Parser encountered an unexpected end of input
    EOF,
    /// Parser encountered an extra token
    ExtraToken(Tok),
    /// Parser encountered an invalid token
    InvalidToken,
    /// Parser encountered an unexpected token
    UnrecognizedToken(char),
    /// Maps to `User` type from `lalrpop-util`
    Lexical(LexicalErrorType),

    SyntaxError(String),
    TypeError(String),
    UnsupportedError,
    Unreachable,
}

#[derive(Debug, PartialEq)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum LexicalErrorType {
    UnrecognizedToken(char),
    OtherError(String),
}

impl From<LalrpopError<Location, Tok, LexicalError>> for ZokError {
    fn from(err: LalrpopError<Location, Tok, LexicalError>) -> Self {
        match err {
            LalrpopError::InvalidToken { location } => ZokError {
                error: ZokErrorType::EOF,
                location,
            },
            LalrpopError::ExtraToken { token } => ZokError {
                error: ZokErrorType::ExtraToken(token.1),
                location: token.0,
            },
            LalrpopError::User { error } => ZokError {
                error: ZokErrorType::Lexical(error.error),
                location: error.location,
            },
            LalrpopError::UnrecognizedToken { token, expected } => ZokError {
                error: ZokErrorType::UnrecognizedToken('c'),
                location: token.0,
            },
            LalrpopError::UnrecognizedEOF { location, .. } => ZokError {
                error: ZokErrorType::EOF,
                location,
            },
        }
    }
}

impl From<ParseIntError> for LexicalError {
    fn from(_err: ParseIntError) -> Self {
        LexicalError {
            error: LexicalErrorType::UnrecognizedToken('c'),
            location: Default::default(),
        }
    }
}

impl Display for ZokError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Display for ZokErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZokErrorType::InvalidToken => write!(f, "Got invalid token"),
            ZokErrorType::UnrecognizedToken(c) => write!(f, "Got unexpected token"),
            _ => write!(f, "Got parser Error"),
        }
    }
}

impl Error for ZokError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
