use super::{operator::Operator, punct::Punct};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TokenData {
    /// A name.
    Name(String),

    /// A character.
    Char(char),

    /// A boolean.
    Boolean(bool),

    /// A string.
    String(String),

    /// A number.
    Number(i32),

    /// An operator. See the `Operator` enum for more information.
    Operator(Operator),

    /// End of file
    EndOfFile,

    /// Punctuation
    Punct(Punct),

    #[default]
    None,
}
