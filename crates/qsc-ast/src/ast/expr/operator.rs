use strum::EnumString;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString, Serialize, Deserialize,
)]
pub enum Operator {
    // Basic Operations
    #[strum(serialize = "+")]
    Add,

    #[strum(serialize = "-")]
    Subtract,

    #[strum(serialize = "*")]
    Multiply,

    #[strum(serialize = "/")]
    Divide,

    #[strum(serialize = "%")]
    Modulo,

    // Basic Assignment Operations
    #[strum(serialize = "=")]
    Assign,

    #[strum(serialize = "+=")]
    AddAssign,

    #[strum(serialize = "-=")]
    SubtractAssign,

    #[strum(serialize = "*=")]
    MultiplyAssign,

    #[strum(serialize = "/=")]
    DivideAssign,

    #[strum(serialize = "%=")]
    ModuloAssign,

    // Logic Operators
    #[strum(serialize = "&&")]
    And,

    #[strum(serialize = "||")]
    Or,

    #[strum(serialize = "!")]
    Not,

    // Bitwise Operators
    #[strum(serialize = "&")]
    BitwiseAnd,

    #[strum(serialize = "|")]
    BitwiseOr,

    #[strum(serialize = "~")]
    BitwiseNot,

    #[strum(serialize = "^")]
    BitwiseXor,

    // Bitwise Assignment Operators
    #[strum(serialize = "&=")]
    BitwiseAndAssign,

    #[strum(serialize = "|=")]
    BitwiseOrAssign,

    #[strum(serialize = "~=")]
    BitwiseNotAssign,

    #[strum(serialize = "^=")]
    BitwiseXorAssign,

    // Equality Operators
    #[strum(serialize = "==")]
    Equal,

    #[strum(serialize = "!=")]
    NotEqual,

    #[strum(serialize = ">")]
    Greater,

    #[strum(serialize = "<")]
    Less,

    #[strum(serialize = ">=")]
    GreaterEqual,

    #[strum(serialize = "<=")]
    LessEqual,
}
