use anyhow::Result;
use qsc_ast::expr::{Expr, ExprKind};
use qsc_tokenizer::{cursor::Cursor, cursor::TokenCursor, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub tokens: TokenCursor,
    pub exprs: Vec<Expr>,
}

impl Lexer {
    pub fn new(cursor: Cursor, data: Vec<Token>) -> Self {
        Lexer {
            tokens: TokenCursor::new(cursor, data),
            exprs: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Expr>> {
        while self.tokens.peek().is_some() {
            self.exprs.push(Expr::parse(&mut self.tokens)?);
        }

        self.exprs = self
            .exprs
            .iter_mut()
            .map(|v| v.clean())
            .filter(|expr| expr.content != ExprKind::None)
            .collect();

        Ok(self.exprs.clone())
    }
}
