use std::fmt::{Display, Formatter};
use crate::file::trace::Trace;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::{Operator, Token, TokenData};


pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token),

    ExpectedToken {
        expected: TokenData,
        received: Token,
    },

    ExpectedIdentifier(Trace),

    ExpectedAnyOperator(Trace),
    ExpectedOperator(Operator, Trace),
    ExpectedAnyKeyword(Trace),
    ExpectedKeyword(Keyword, Trace),

    OpenParenthetical(Trace),

    NonUnaryOperator(Operator, Trace),

    ExpectedFunctionBody(Trace),
}

/// TODO implement more detailed error messages
impl ParserError {
    /// Gets the trace / place in file for error
    fn trace(&self) -> Vec<&Trace> {
        match self {
            ParserError::UnexpectedToken(Token(_, trace)) => vec![trace],

            ParserError::ExpectedToken {
                expected: _,
                received
            } => vec![received.trace()],

            ParserError::OpenParenthetical(trace) => vec![trace],
            ParserError::NonUnaryOperator(_, trace) => vec![trace],
            ParserError::ExpectedIdentifier(trace) => vec![trace],
            ParserError::ExpectedAnyOperator(trace) => vec![trace],
            ParserError::ExpectedOperator(_, trace) => vec![trace],
            ParserError::ExpectedAnyKeyword(trace) => vec![trace],
            ParserError::ExpectedKeyword(_, trace) => vec![trace],
            ParserError::ExpectedFunctionBody(trace) => vec![trace],
        }
    }
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // FIXME display trace from file
        f.write_fmt(format_args!("{self:#?}"))
    }
}
