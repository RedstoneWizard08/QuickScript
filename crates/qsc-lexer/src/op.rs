use pest::iterators::Pair;
use qsc_ast::ast::expr::{binary::BinaryExpr, operator::Operator};
use qsc_core::{
    conv::IntoSourceSpan,
    error::{lexer::LexerError, Result},
};

use crate::{lexer::Lexer, parser::Rule};

impl<'i> Lexer {
    pub fn binary_op(&self, pair: Pair<'i, Rule>) -> Result<BinaryExpr> {
        let mut inner = pair.clone().into_inner();

        let span = pair.as_span().into();
        let lhs = self.parse(inner.next().unwrap())?;
        let op = inner.next().unwrap().as_str().trim().to_string();
        let rhs = self.parse(inner.next().unwrap())?;

        Ok(match op.as_str() {
            "+" => BinaryExpr {
                operator: Operator::Add,
                span,
                lhs,
                rhs,
            },

            "-" => BinaryExpr {
                operator: Operator::Subtract,
                span,
                lhs,
                rhs,
            },

            "*" => BinaryExpr {
                operator: Operator::Multiply,
                span,
                lhs,
                rhs,
            },

            "/" => BinaryExpr {
                operator: Operator::Divide,
                span,
                lhs,
                rhs,
            },

            "%" => BinaryExpr {
                operator: Operator::Modulo,
                span,
                lhs,
                rhs,
            },

            "&&" => BinaryExpr {
                operator: Operator::And,
                span,
                lhs,
                rhs,
            },

            "||" => BinaryExpr {
                operator: Operator::Or,
                span,
                lhs,
                rhs,
            },

            "^" => BinaryExpr {
                operator: Operator::BitwiseXor,
                span,
                lhs,
                rhs,
            },

            "!" => BinaryExpr {
                operator: Operator::Not,
                span,
                lhs,
                rhs,
            },

            "&" => BinaryExpr {
                operator: Operator::BitwiseAnd,
                span,
                lhs,
                rhs,
            },

            "|" => BinaryExpr {
                operator: Operator::BitwiseOr,
                span,
                lhs,
                rhs,
            },

            "~" => BinaryExpr {
                operator: Operator::BitwiseNot,
                span,
                lhs,
                rhs,
            },

            "==" => BinaryExpr {
                operator: Operator::Equal,
                span,
                lhs,
                rhs,
            },

            "!=" => BinaryExpr {
                operator: Operator::NotEqual,
                span,
                lhs,
                rhs,
            },

            ">" => BinaryExpr {
                operator: Operator::Greater,
                span,
                lhs,
                rhs,
            },

            "<" => BinaryExpr {
                operator: Operator::Less,
                span,
                lhs,
                rhs,
            },

            ">=" => BinaryExpr {
                operator: Operator::GreaterEqual,
                span,
                lhs,
                rhs,
            },

            "<=" => BinaryExpr {
                operator: Operator::LessEqual,
                span,
                lhs,
                rhs,
            },

            "=>" => BinaryExpr {
                operator: Operator::Assign,
                span,
                lhs,
                rhs,
            },

            "+=" => BinaryExpr {
                operator: Operator::AddAssign,
                span,
                lhs,
                rhs,
            },

            "-=" => BinaryExpr {
                operator: Operator::SubtractAssign,
                span,
                lhs,
                rhs,
            },

            "*=" => BinaryExpr {
                operator: Operator::MultiplyAssign,
                span,
                lhs,
                rhs,
            },

            "/=" => BinaryExpr {
                operator: Operator::DivideAssign,
                span,
                lhs,
                rhs,
            },

            "%=" => BinaryExpr {
                operator: Operator::ModuloAssign,
                span,
                lhs,
                rhs,
            },

            "&=" => BinaryExpr {
                operator: Operator::BitwiseAndAssign,
                span,
                lhs,
                rhs,
            },

            "|=" => BinaryExpr {
                operator: Operator::BitwiseOrAssign,
                span,
                lhs,
                rhs,
            },

            "~=" => BinaryExpr {
                operator: Operator::BitwiseNotAssign,
                span,
                lhs,
                rhs,
            },

            "^=" => BinaryExpr {
                operator: Operator::BitwiseXorAssign,
                span,
                lhs,
                rhs,
            },

            val => {
                return Err(LexerError {
                    src: self.err_src.clone(),
                    location: pair.as_span().into_source_span(),
                    error: miette!("Unsupported operator: {}", val),
                }
                .into())
            }
        })
    }
}
