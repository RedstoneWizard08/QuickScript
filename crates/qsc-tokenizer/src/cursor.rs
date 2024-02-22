use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor {
    pub file: String,
    pub all_data: String,
    pub data: Vec<char>,
    pub position: usize,
}

impl Cursor {
    pub fn new(file: String, data: Vec<char>) -> Self {
        Self {
            data: data.clone(),
            file,
            all_data: String::from_iter(data),
            position: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.data.is_empty() {
            return None;
        }

        self.position += 1;

        Some(self.data.remove(0))
    }

    pub fn peek(&self) -> Option<char> {
        self.data.first().cloned()
    }

    pub fn peek_at(&self, index: usize) -> Option<char> {
        self.data.get(index).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenCursor {
    pub all_data: Vec<Token>,
    pub data: Vec<Token>,
    pub position: usize,
    pub cursor: Cursor,
}

impl TokenCursor {
    pub fn new(cursor: Cursor, data: Vec<Token>) -> Self {
        Self {
            data: data.clone(),
            all_data: data,
            position: 0,
            cursor,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.data.is_empty() {
            return None;
        }

        self.position += 1;

        Some(self.data.remove(0))
    }

    pub fn peek(&self) -> Option<Token> {
        self.data.first().cloned()
    }

    pub fn peek_at(&self, index: usize) -> Option<Token> {
        self.data.get(index).cloned()
    }

    pub fn remove(&mut self, index: usize) -> Option<Token> {
        if index >= self.data.len() {
            return None;
        }

        Some(self.data.remove(index))
    }
}
