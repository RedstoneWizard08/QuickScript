use miette::{IntoDiagnostic, NamedSource};
use pest::{iterators::Pair, Parser};

use qsc_ast::ast::{
    decl::DeclarationNode,
    expr::{unary::UnaryExpr, ExpressionNode},
    literal::{
        boolean::BoolNode, char::CharNode, float::FloatNode, int::IntNode, string::StringNode,
        LiteralNode,
    },
    node::{block::Block, data::NodeData, sym::SymbolNode, Node},
    stmt::{ret::ReturnNode, StatementNode},
    AbstractTree,
};

use qsc_core::{
    conv::IntoSourceSpan,
    error::{lexer::LexerError, Result},
};

use crate::parser::{CodeParser, Rule};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    pub src: String,
    pub tree: AbstractTree,
    pub err_src: NamedSource<String>,
}

impl<'i> Lexer {
    pub fn new(name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        let src = source.as_ref().to_string();
        let name = name.as_ref().to_string();
        let err_src = NamedSource::new(name.clone(), src.clone());

        Self {
            tree: AbstractTree::new(name, src.clone()),
            src,
            err_src,
        }
    }

    pub fn lex(&mut self) -> Result<AbstractTree> {
        let data = CodeParser::parse(Rule::main, self.src.as_str()).into_diagnostic()?;

        for pair in data {
            if let Rule::main = pair.as_rule() {
                for pair in pair.into_inner() {
                    self.tree.data.push(self.parse(pair)?);
                }
            }
        }

        Ok(self.tree.clone())
    }

    pub fn parse(&self, pair: Pair<'i, Rule>) -> Result<Node> {
        Ok(Node {
            span: pair.as_span().into(),
            data: Box::new(self.parse_data(pair)?),
        })
    }

    pub fn parse_data(&self, pair: Pair<'i, Rule>) -> Result<NodeData> {
        Ok(match pair.as_rule() {
            Rule::function => {
                NodeData::Declaration(DeclarationNode::Function(self.function(pair)?))
            }

            Rule::binary_op => NodeData::Expr(ExpressionNode::Binary(self.binary_op(pair)?)),
            Rule::call => NodeData::Statement(StatementNode::Call(self.call(pair)?)),
            Rule::var => NodeData::Declaration(DeclarationNode::Variable(self.var(pair)?)),
            Rule::r#type => NodeData::Type(self.ty(pair)?),

            Rule::ident => NodeData::Symbol(SymbolNode {
                span: pair.as_span().into(),
                value: pair.as_str().trim().to_string(),
            }),

            Rule::unary_op => NodeData::Expr(ExpressionNode::Unary(UnaryExpr {
                span: pair.as_span().into(),
                negative: pair.as_str().trim().starts_with("-"),
                value: self.parse(pair.into_inner().last().unwrap())?,
            })),

            // Redirects
            Rule::literal => self.parse_data(pair.into_inner().next().unwrap())?,

            // Primitives (literals)
            Rule::char => NodeData::Literal(LiteralNode::Char(CharNode {
                span: pair.as_span().into(),
                value: self
                    .interp_literal(pair.as_str().trim().trim_matches('\''))
                    .chars()
                    .nth(0)
                    .unwrap(),
            })),

            Rule::string => NodeData::Literal(LiteralNode::String(StringNode {
                span: pair.as_span().into(),
                value: self.interp_literal(pair.as_str().trim().trim_matches('"')),
            })),

            Rule::float => NodeData::Literal(LiteralNode::Float(FloatNode {
                span: pair.as_span().into(),
                value: pair.as_str().trim().parse().unwrap(),
            })),

            Rule::int => NodeData::Literal(LiteralNode::Int(IntNode {
                span: pair.as_span().into(),
                value: pair.as_str().trim().parse().unwrap(),
            })),

            Rule::bool => NodeData::Literal(LiteralNode::Bool(BoolNode {
                span: pair.as_span().into(),
                value: pair.as_str().trim().parse().unwrap(),
            })),

            // Groups
            Rule::term => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::call => NodeData::Statement(StatementNode::Call(self.call(pair)?)),
                    Rule::literal => self.parse_data(pair)?,
                    Rule::ident => self.parse_data(pair)?,

                    _ => {
                        return Err(LexerError {
                            src: self.err_src.clone(),
                            location: pair.as_span().into_source_span(),
                            error: miette!("Unsupported pair: {:?}", pair),
                        }
                        .into())
                    }
                }
            }

            Rule::expr => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::term => self.parse_data(pair)?,
                    Rule::binary_op => {
                        NodeData::Expr(ExpressionNode::Binary(self.binary_op(pair)?))
                    }

                    _ => {
                        return Err(LexerError {
                            src: self.err_src.clone(),
                            location: pair.as_span().into_source_span(),
                            error: miette!("Unsupported pair: {:?}", pair),
                        }
                        .into())
                    }
                }
            }

            Rule::number => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::int => self.parse_data(pair)?,
                    Rule::float => self.parse_data(pair)?,

                    val => {
                        return Err(LexerError {
                            src: self.err_src.clone(),
                            location: pair.as_span().into_source_span(),
                            error: miette!("Unsupported number child: {:?}", val),
                        }
                        .into())
                    }
                }
            }

            Rule::statement => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::real_stmt => self.parse_data(pair)?,

                    _ => {
                        return Err(LexerError {
                            src: self.err_src.clone(),
                            location: pair.as_span().into_source_span(),
                            error: miette!("Unsupported pair: {:?}", pair),
                        }
                        .into())
                    }
                }
            }

            Rule::real_stmt => {
                let pair = pair.into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::ret => self.parse_data(pair)?,
                    Rule::var => self.parse_data(pair)?,
                    Rule::expr => self.parse_data(pair)?,
                    Rule::block => self.parse_data(pair)?,
                    Rule::conditional => self.parse_data(pair)?,

                    _ => {
                        return Err(LexerError {
                            src: self.err_src.clone(),
                            location: pair.as_span().into_source_span(),
                            error: miette!("Unsupported pair: {:?}", pair),
                        }
                        .into())
                    }
                }
            }

            Rule::block => NodeData::Block(Block {
                span: pair.as_span().into(),
                data: pair
                    .into_inner()
                    .map(|pair| self.parse(pair).unwrap())
                    .collect::<Vec<_>>(),
            }),

            Rule::conditional => {
                NodeData::Statement(StatementNode::Condition(self.condition(pair)?))
            }

            // Simple ones
            Rule::ret => NodeData::Statement(StatementNode::Return(ReturnNode {
                span: pair.as_span().into(),
                value: if let Some(pair) = pair.into_inner().next() {
                    Some(self.parse(pair)?)
                } else {
                    None
                },
            })),

            Rule::EOI => NodeData::EOI,

            val => {
                return Err(LexerError {
                    src: self.err_src.clone(),
                    location: pair.as_span().into_source_span(),
                    error: miette!("Unsupported pair: {:?}", val),
                }
                .into())
            }
        })
    }
}
