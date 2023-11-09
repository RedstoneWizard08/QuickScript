pub const OPERATORS: [char; 31] = [
    '+', '-', '*', '/', ':', ';', '\'', '"', '(', ')', '[', ']', '{', '}', '#', '@', '&', '|', '^',
    '%', '<', '>', '.', ',', '?', '$', '!', '\\', '`', '~', '=',
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Operator {
    Add = '+' as u8,
    Subtract = '-' as u8,
    Multiply = '*' as u8,
    Divide = '/' as u8,
    Colon = ':' as u8,
    Semicolon = ';' as u8,
    SingleQuote = '\'' as u8,
    DoubleQuote = '"' as u8,
    OpenParens = '(' as u8,
    CloseParens = ')' as u8,
    OpenSqBracket = '[' as u8,
    CloseSqBracket = ']' as u8,
    OpenCurly = '{' as u8,
    CloseCurly = '}' as u8,
    Hash = '#' as u8,
    At = '@' as u8,
    BitAnd = '&' as u8,
    BitOr = '|' as u8,
    Xor = '^' as u8,
    Modulo = '%' as u8,
    Lt = '<' as u8,
    Gt = '>' as u8,
    Decimal = '.' as u8,
    Comma = ',' as u8,
    Try = '?' as u8,
    Dollar = '$' as u8,
    Bang = '!' as u8,
    Escape = '\\' as u8,
    Backtick = '`' as u8,
    Tilde = '~' as u8,
    Equals = '=' as u8,
}

impl Operator {
    pub fn as_char(self) -> char {
        self.into()
    }
}

impl Into<char> for Operator {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Operator::Add,
            '-' => Operator::Subtract,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            ':' => Operator::Colon,
            ';' => Operator::Semicolon,
            '\'' => Operator::SingleQuote,
            '"' => Operator::DoubleQuote,
            '(' => Operator::OpenParens,
            ')' => Operator::CloseParens,
            '[' => Operator::OpenSqBracket,
            ']' => Operator::CloseSqBracket,
            '{' => Operator::OpenCurly,
            '}' => Operator::CloseCurly,
            '#' => Operator::Hash,
            '@' => Operator::At,
            '&' => Operator::BitAnd,
            '|' => Operator::BitOr,
            '^' => Operator::Xor,
            '%' => Operator::Modulo,
            '<' => Operator::Lt,
            '>' => Operator::Gt,
            '.' => Operator::Decimal,
            ',' => Operator::Comma,
            '?' => Operator::Try,
            '$' => Operator::Dollar,
            '!' => Operator::Bang,
            '\\' => Operator::Escape,
            '`' => Operator::Backtick,
            '~' => Operator::Tilde,
            '=' => Operator::Equals,

            _ => panic!("Unknown operator!"),
        }
    }
}
