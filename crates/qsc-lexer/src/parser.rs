use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use qsc_ast::{
    expr::{Expr, ExprKind},
    literal::Literal,
};

#[derive(Parser)]
#[grammar = "quickscript.pest"]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self
    }

    pub fn lex(&self, source: impl AsRef<str>) -> Result<Vec<Expr>> {
        let src = source.as_ref();
        let data = Self::parse(Rule::main, src)?;
        let mut exprs = Vec::new();

        for pair in data {
            if let Rule::main = pair.as_rule() {
                for pair in pair.into_inner() {
                    if let Rule::function = pair.as_rule() {
                        exprs.push(self.parse_expr(pair));
                    }
                }
            }
        }

        Ok(exprs)
    }

    pub fn parse_expr<'i>(&self, pair: Pair<'i, Rule>) -> Expr {
        let kind = self.parse_kind(pair.clone());

        Expr {
            content: kind,
            position: pair.as_span().start()..pair.as_span().end(),
        }
    }

    pub fn parse_kind<'i>(&self, pair: Pair<'i, Rule>) -> ExprKind {
        match pair.as_rule() {
            Rule::function => self.function(&pair),
            Rule::binary_op => self.binary_op(&pair),
            Rule::call => self.call(&pair),
            Rule::var => self.var(&pair),
            Rule::r#type => self.r#type(&pair),
            Rule::ident => ExprKind::Identifer(pair.as_str().trim().to_string()),

            Rule::unary_op => ExprKind::Unary(
                pair.as_str().trim().starts_with("-"),
                Box::new(self.parse_kind(pair.into_inner().last().unwrap())),
            ),

            // Redirects
            Rule::literal => self.parse_kind(pair.into_inner().next().unwrap()),

            // Primitives (literals)
            Rule::char => {
                ExprKind::Literal(Literal::Char(pair.as_str().trim().chars().nth(0).unwrap()))
            }

            Rule::string => ExprKind::Literal(Literal::String(pair.as_str().trim().to_string())),
            Rule::float => ExprKind::Literal(Literal::Float(pair.as_str().trim().parse().unwrap())),
            Rule::int => ExprKind::Literal(Literal::Integer(pair.as_str().trim().parse().unwrap())),

            Rule::bool => {
                ExprKind::Literal(Literal::Boolean(pair.as_str().trim().parse().unwrap()))
            }

            // Groups
            Rule::term => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::call => self.call(&pair),
                    Rule::literal => self.parse_kind(pair),
                    Rule::ident => self.parse_kind(pair),

                    _ => unreachable!(),
                }
            }

            Rule::expr => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::term => self.parse_kind(pair),
                    Rule::binary_op => self.binary_op(&pair),

                    _ => unreachable!(),
                }
            }

            Rule::number => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::int => self.parse_kind(pair),
                    Rule::float => self.parse_kind(pair),

                    _ => unreachable!(),
                }
            }

            Rule::statement => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::real_stmt => self.parse_kind(pair),

                    _ => ExprKind::None,
                }
            }

            Rule::real_stmt => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::ret => self.parse_kind(pair),
                    Rule::var => self.parse_kind(pair),
                    Rule::expr => self.parse_kind(pair),
                    Rule::block => self.parse_kind(pair),

                    _ => unreachable!(),
                }
            }

            Rule::block => ExprKind::Block(
                pair.into_inner()
                    .map(|pair| self.parse_expr(pair))
                    .collect::<Vec<_>>(),
            ),

            // Simple ones
            Rule::ret => ExprKind::Return(if let Some(pair) = pair.into_inner().nth(1) {
                if pair.as_rule() == Rule::ret {
                    Some(Box::new(self.parse_expr(pair)))
                } else {
                    None
                }
            } else {
                None
            }),

            _ => ExprKind::None,
        }
    }
}
