use core::fmt;

#[derive(Debug, Clone)]
pub enum CalcError {
    InvalidCharacter(char),
    UnexpectedToken,
    UnexpectedEnd,
    InvalidNumber,
    UnknownFunction(String),
    DivisionByZero,
    DomainError,
    InvalidExpression,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(f, "Invalid character: {c}"),
            Self::UnexpectedToken => write!(f, "Unexpected token"),
            Self::UnexpectedEnd => write!(f, "Unexpected end"),
            Self::InvalidNumber => write!(f, "Invalid number"),
            Self::UnknownFunction(name) => write!(f, "Unknown function: {name}"),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::DomainError => write!(f, "Domain error"),
            Self::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}
