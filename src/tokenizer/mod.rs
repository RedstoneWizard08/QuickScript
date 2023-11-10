use self::{
    consumer::Cursor,
    token::{
        operator::{Operator, OPERATORS},
        ttype::TokenType,
    },
};

pub mod consumer;
pub mod parse;
pub mod recurse;
pub mod token;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tokenizer {
    pub content: Vec<char>,
    pub tokens: Vec<TokenType>,
}

impl Tokenizer {
    pub fn tokenize(&mut self) {
        let mut iter = Cursor::new(self.content.clone());

        while let Some(ch) = iter.next() {
            if ch.is_whitespace() {
                continue;
            }

            if ch == '"' {
                self.tokens.push(TokenType::read_string(&mut iter));
            } else if OPERATORS.contains(&ch) {
                self.tokens.push(TokenType::Operator(Operator::from(ch)));
            } else if ch.is_numeric() {
                let mut buf = vec![ch.to_digit(10).unwrap() as u8];

                while let Some(ch) = iter.next() {
                    if !ch.is_numeric() {
                        iter.prev();

                        break;
                    }

                    buf.push(ch.to_digit(10).unwrap() as u8);
                }

                self.tokens.push(TokenType::Number(buf));
            } else {
                let mut buf = vec![ch];

                while let Some(ch) = iter.next() {
                    if OPERATORS.contains(&ch) || ch == '"' || ch.is_whitespace() {
                        iter.prev();

                        break;
                    }

                    buf.push(ch);
                }

                self.tokens.push(TokenType::Name(buf));
            }
        }
    }
}

impl From<Vec<char>> for Tokenizer {
    fn from(value: Vec<char>) -> Self {
        Self {
            content: value,
            tokens: Vec::new(),
        }
    }
}

impl From<String> for Tokenizer {
    fn from(value: String) -> Self {
        Self {
            content: value.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
        }
    }
}
