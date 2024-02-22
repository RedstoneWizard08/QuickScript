use std::fmt::{self, Display, Formatter};

pub const KNOWN_OPERATORS: [&str; 28] = [
    "+", "-", "*", "/", "%", "=", "==", "!=", ">", "<", ">=", "<=", "&&", "||", "!", "&", "|", "~",
    "^", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "~=", "^=",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    Xor,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseNotAssign,
    XorAssign,

    #[default]
    None,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Assign => write!(f, "="),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Greater => write!(f, ">"),
            Self::Less => write!(f, "<"),
            Self::GreaterEqual => write!(f, ">="),
            Self::LessEqual => write!(f, "<="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
            Self::Not => write!(f, "!"),
            Self::BitwiseAnd => write!(f, "&"),
            Self::BitwiseOr => write!(f, "|"),
            Self::BitwiseNot => write!(f, "~"),
            Self::Xor => write!(f, "^"),
            Self::AddAssign => write!(f, "+="),
            Self::SubAssign => write!(f, "-="),
            Self::MulAssign => write!(f, "*="),
            Self::DivAssign => write!(f, "/="),
            Self::ModAssign => write!(f, "%="),
            Self::BitwiseAndAssign => write!(f, "&="),
            Self::BitwiseOrAssign => write!(f, "|="),
            Self::BitwiseNotAssign => write!(f, "~="),
            Self::XorAssign => write!(f, "^="),
            Self::None => write!(f, "None"),
        }
    }
}
