use std::process::exit;

use serde::Serialize;

use crate::{
    cursor::{Cursor, Reader},
    token::{Token, EXPRESSIONS, TOKENS},
};

/// A parser/tokenizer for an unknown semicolon-based
/// syntax tree. Can identify strings, numbers, literals,
/// identifiers, and some other simple tokens.
#[derive(Debug, Clone, Serialize)]
pub struct Parser {
    pub code: Cursor<String>,
    pub tokens: Vec<Token>,
}

impl Parser {
    /// Create a new parser. Takes in a code string
    /// which will be turned into a Cursor (a fancy
    /// iterator) to make it easier to break it down
    /// into a list of tokens which will later be
    /// used to create a syntax tree.
    pub fn new(code: String) -> Self {
        Self {
            code: Cursor::new(code),
            tokens: Vec::new(),
        }
    }

    /// Just a wrapper for more concise code down the line,
    /// as Rust gets confused since I implemented Reader<T>
    /// for T and String specifically.
    fn read(&mut self) -> char {
        self.code.read()
    }

    /// Read and parse the given code. Will use the internal
    /// cursor to read it character by character and turn it
    /// into Tokens, which will be pushed into the Vec of
    /// tokens internally. Those can then be accessed when
    /// we use it to build the syntax tree.
    pub fn parse(&mut self) -> Vec<Token> {
        while self.code.has_next() {
            let mut ch = self.read();

            if ch == '"' {
                // We are starting a string.
                let mut buf = String::new();

                while ch != '"' && self.code.has_next() {
                    ch = self.read();

                    if ch != '"' {
                        buf.push(ch);
                    }
                }

                // Add the token.
                self.tokens.push(TOKENS.get("STRING").unwrap().create(buf));

                // Skip any other checks.
                continue;
            }

            if ch == '\'' {
                // We are starting a single character.
                let mut buf = String::new();

                // Since there may be an escape code, we still have to loop.
                while ch != '\'' && self.code.has_next() {
                    ch = self.read();

                    if ch != '\'' {
                        buf.push(ch);
                    }
                }

                // Check if it is now more than one character.
                if buf.len() > 1 {
                    eprintln!(
                        "Invalid value for character literal: {} (index {})",
                        buf,
                        self.code.position - (buf.len() + 1)
                    );
                    eprintln!("Character literals can only be one character long!");
                    exit(1);
                }

                // Add the token.
                self.tokens.push(TOKENS.get("CHAR").unwrap().create(buf));

                // Skip any other checks.
                continue;
            }

            if ch.is_numeric() {
                // We are starting some kind of number.
                let mut buf = String::new();
                let mut points = 0;

                buf.push(ch);

                while (ch.is_numeric() || (ch == '.' && points <= 1)) && self.code.has_next() {
                    ch = self.read();

                    if ch == '.' {
                        points += 1;
                        buf.push(ch);

                        if points > 1 {
                            eprintln!(
                                "Invalid syntax for float: {} (index {})",
                                buf,
                                self.code.position - (buf.len() + 1)
                            );
                            eprintln!("Floating-point numbers can only have one decimal point!");
                            exit(1);
                        }
                    } else if ch.is_numeric() {
                        buf.push(ch);
                    }
                }

                // Account for the character that was read at the end of the loop.
                self.code.position -= 1;

                // Add the token.
                if points > 0 {
                    // This is a float.
                    self.tokens.push(TOKENS.get("FLOAT").unwrap().create(buf));
                } else {
                    // This is an int.
                    self.tokens.push(TOKENS.get("INTEGER").unwrap().create(buf));
                }

                // Skip any other checks.
                continue;
            }

            if EXPRESSIONS.contains(&ch.to_string().as_str()) {
                // This is an expression.

                // Add the token.
                self.tokens
                    .push(TOKENS.get("EXPR").unwrap().create(ch.to_string()));

                // Skip any other checks.
                continue;
            }

            if ch.is_alphanumeric() {
                // This is an identifier.
                let mut buf = String::new();

                buf.push(ch);

                while ch.is_alphanumeric() && self.code.has_next() {
                    ch = self.read();

                    if ch.is_alphanumeric() {
                        buf.push(ch);
                    }
                }

                // Account for the character that was read at the end of the loop.
                self.code.position -= 1;

                // Add the token.
                self.tokens.push(TOKENS.get("IDENT").unwrap().create(buf));

                // Skip any other checks.
                continue;
            }
        }

        self.tokens.clone()
    }
}
