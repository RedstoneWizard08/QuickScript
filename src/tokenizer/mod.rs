use self::{cursor::Cursor, data::TokenData, token::Token};

pub mod cursor;
pub mod data;
pub mod error;
pub mod operator;
pub mod punct;
pub mod token;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    pub data: Cursor,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(file: impl AsRef<str>, data: String) -> Self {
        Self {
            data: Cursor::new(file.as_ref().to_string(), data.chars().collect()),
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while let Some(ch) = self.data.peek() {
            if ch.is_whitespace() {
                self.data.next();
                continue;
            }

            self.tokens.push(Token::read(&mut self.data));
        }

        self.tokens = self
            .tokens
            .iter()
            .cloned()
            .filter(|v| v.content != TokenData::None)
            .collect();

        self.tokens.clone()
    }
}
