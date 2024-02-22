use super::{operator::Operator, punct::Punct};
use qsc_core::enum_export;
use std::fmt::{self, Display, Formatter};

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

enum_export!(TokenData, as_name, Name, String);
enum_export!(TokenData, as_char, Char, char);
enum_export!(TokenData, as_boolean, Boolean, bool);
enum_export!(TokenData, as_string, String, String);
enum_export!(TokenData, as_number, Number, i32);
enum_export!(TokenData, as_operator, Operator, Operator);
enum_export!(TokenData, as_punct, Punct, Punct);

impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Char(c) => write!(f, "{}", c),
            Self::Name(n) => write!(f, "{}", n),
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Operator(o) => write!(f, "{}", o),
            Self::Punct(p) => write!(f, "{}", p),
            Self::EndOfFile => write!(f, "EOF"),
            Self::None => write!(f, "None"),
        }
    }
}
