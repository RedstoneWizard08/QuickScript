use crate::tokenizer::consumer::Cursor;

use super::ttype::TokenType;

impl TokenType {
    pub fn read_string(buf: &mut Cursor<char>) -> TokenType {
        let mut s = Vec::new();

        while let Some(ch) = buf.next() {
            if ch == '"' && s.last() != Some(&'\\') {
                break;
            }

            s.push(ch);
        }

        TokenType::String(s)
    }
}
