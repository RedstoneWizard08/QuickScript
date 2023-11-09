use anyhow::Result;

use crate::tokenizer::consumer::Cursor;

use super::operator::Operator;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TokenType {
    Name(Vec<char>),
    String(Vec<char>),
    Number(Vec<u8>),
    Operator(Operator),
    None,
}

impl TokenType {
    pub fn read_operator(buf: &mut Cursor<char>) -> TokenType {
        TokenType::Operator(Operator::from(buf.next().unwrap()))
    }

    pub fn get_name(&self) -> Result<String> {
        match self.clone() {
            Self::Name(val) => Ok(String::from_iter(val)),
            _ => Err(anyhow!("TokenType is not TokenType::Name!")),
        }
    }

    pub fn get_string(&self) -> Result<String> {
        match self.clone() {
            Self::String(val) => Ok(String::from_iter(val)),
            _ => Err(anyhow!("TokenType is not TokenType::String!")),
        }
    }

    pub fn get_number(&self) -> Result<i32> {
        match self.clone() {
            Self::Number(val) => Ok(String::from_iter(
                val.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            )
            .parse()?),
            _ => Err(anyhow!("TokenType is not TokenType::Number!")),
        }
    }

    pub fn get_operator(&self) -> Result<Operator> {
        match self.clone() {
            Self::Operator(op) => Ok(op),
            _ => Err(anyhow!("TokenType is not TokenType::Operator!")),
        }
    }

    pub fn is_number(&self) -> bool {
        match self.clone() {
            Self::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_name(&self) -> bool {
        match self.clone() {
            Self::Name(_) => true,
            _ => false,
        }
    }
}
