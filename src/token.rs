use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// A token. Contains a name (string identifier),
/// an id (numerical identifier), a pretty name
/// (a readable string identifier), and an optional
/// value to contain any data these might represent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: &'static str,
    pub pretty_name: &'static str,
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
            name: "STR_LIT",
            pretty_name: "String Literal",
            value: None,
        }),

        // Single characters
        ("CHAR", Token {
            id: 1,
            name: "CHAR_LIT",
            pretty_name: "Character Literal",
            value: None,
        }),

        // Integers
        ("INTEGER", Token {
            id: 2,
            name: "INT_LIT",
            pretty_name: "Integer Literal",
            value: None,
        }),

        // Floating-point numbers
        ("FLOAT", Token {
            id: 3,
            name: "FLOAT_LIT",
            pretty_name: "Float Literal",
            value: None,
        }),

        // Identifiers (names of things, including keywords)
        ("IDENT", Token {
            id: 4,
            name: "IDENT",
            pretty_name: "Identifier",
            value: None,
        }),

        // Expressions (+, -, *, /, ^, &, |, etc.)
        ("EXPR", Token {
            id: 5,
            name: "EXPR",
            pretty_name: "Expression",
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
    ];
}
