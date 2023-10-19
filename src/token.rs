use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// A token. Contains a name (string identifier),
/// an id (numerical identifier), a pretty name
/// (a readable string identifier), and an optional
/// value to contain any data these might represent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub name: String,
    pub pretty_name: String,
    pub id: i8,
    pub value: Option<String>,
}

impl Token {
    /// Creates a token with the provided value
    /// based on this one.
    pub fn create(&self, value: String) -> Token {
        Token {
            name: self.name.clone(),
            pretty_name: self.pretty_name.clone(),
            id: self.id,
            value: Some(value),
        }
    }

    pub fn as_string(&self) -> String {
        match self.name.as_str() {
            "STR_LIT" => format!("\"{}\"", self.value.clone().unwrap()),
            "CHAR_LIT" => format!("'{}'", self.value.clone().unwrap()),
            _ => self.value.clone().unwrap(),
        }
    }
}

lazy_static! {
    /// A list of pre-defined primitive token types.
    /// Pre-defined tokens:
    /// - "STRING": Strings
    /// - "CHAR": Single characters
    /// - "INTEGER": Integers
    /// - "FLOAT": Floating-point numbers
    /// - "IDENT": Identifiers (names of things, including keywords)
    /// - "EXPR": Expressions (+, -, *, /, &, &, |, etc.)
    pub static ref TOKENS: HashMap<&'static str, Token> = HashMap::from([
        // Strings
        ("STRING", Token {
            id: 0,
            name: String::from("STR_LIT"),
            pretty_name: String::from("String Literal"),
            value: None,
        }),

        // Single characters
        ("CHAR", Token {
            id: 1,
            name: String::from("CHAR_LIT"),
            pretty_name: String::from("Character Literal"),
            value: None,
        }),

        // Integers
        ("INTEGER", Token {
            id: 2,
            name: String::from("INT_LIT"),
            pretty_name: String::from("Integer Literal"),
            value: None,
        }),

        // Floating-point numbers
        ("FLOAT", Token {
            id: 3,
            name: String::from("FLOAT_LIT"),
            pretty_name: String::from("Float Literal"),
            value: None,
        }),

        // Identifiers (names of things, including keywords)
        ("IDENT", Token {
            id: 4,
            name: String::from("IDENT"),
            pretty_name: String::from("Identifier"),
            value: None,
        }),

        // Expressions (+, -, *, /, ^, &, |, etc.)
        ("EXPR", Token {
            id: 5,
            name: String::from("EXPR"),
            pretty_name: String::from("Expression"),
            value: None,
        }),
    ]);

    /// A list of all the currently known expressions
    /// for syntax.
    pub static ref EXPRESSIONS: Vec<&'static str> = vec![
        "+",
        "-",
        "*",
        "/",
        "^",
        "&",
        "|",
        ":",
        "=",
        "(",
        ")",
        "{",
        "}",
        "[",
        "]",
        "!",
        "<",
        ">",
        ",",
        "_",
        ";",
        ".",
    ];
}
