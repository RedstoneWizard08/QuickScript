use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Punct {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    DotDot,

    #[default]
    None,
}

impl Display for Punct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::Semicolon => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::DotDot => write!(f, ".."),
            Self::None => write!(f, "None"),
        }
    }
}
