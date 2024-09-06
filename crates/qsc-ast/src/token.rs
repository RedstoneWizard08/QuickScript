use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Token<'s> {
    Control(char),
    Operation(&'s str),
    Ident(&'s str),
    Value(Value<'s>),

    Fn,
    Let,
    Mut,
    If,
    Else,
    Extern,
    Return,
}

impl<'s> Display for Token<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Control(x) => write!(f, "{}", x),
            Self::Operation(x) => write!(f, "{}", x),
            Self::Ident(x) => write!(f, "{}", x),
            Self::Value(x) => write!(f, "{}", x),

            Self::Fn => write!(f, "fn"),
            Self::Let => write!(f, "let"),
            Self::Mut => write!(f, "mut"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Extern => write!(f, "extern"),
            Self::Return => write!(f, "return"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Value<'s> {
    Bool(bool),
    String(&'s str),
    Int(i64),
    Float(f64),
    Char(char),
}

impl<'s> Display for Value<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(x) => write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
            Self::Char(x) => write!(f, "{}", x),
        }
    }
}
