use im_rc::HashMap;
use qsc_ast::ast::{
    decl::{var::VariableNode, DeclarationNode},
    expr::{binary::BinaryExpr, ExpressionNode},
    node::{data::NodeData, sym::SymbolNode},
    stmt::{call::CallNode, cond::ConditionalNode, ret::ReturnNode, StatementNode},
    AbstractTree,
};

use crate::util::Spanned;

#[derive(Debug, Clone)]
pub enum LangCompletionItem {
    Variable(String),
    Function(String, Vec<String>),
}

pub fn completion(ast: &AbstractTree, ident_offset: usize) -> HashMap<String, LangCompletionItem> {
    let mut map = HashMap::new();

    for (_, v) in ast.functions().iter() {
        if v.span.end < ident_offset {
            map.insert(
                v.name.clone(),
                LangCompletionItem::Function(
                    v.name.clone(),
                    v.args.clone().into_iter().map(|arg| arg.name).collect(),
                ),
            );
        }
    }

    // collect params variable
    for (_, v) in ast.functions().iter() {
        if v.span.end > ident_offset && v.span.start < ident_offset {
            // log::debug!("this is completion from body {}", name);
            v.args.iter().for_each(|arg| {
                map.insert(
                    arg.name.clone(),
                    LangCompletionItem::Variable(arg.name.clone()),
                );
            });

            get_completion_of(
                &(NodeData::Block(v.content.clone()), v.span.clone()),
                &mut map,
                ident_offset,
            );
        }
    }
    map
}

pub fn get_completion_of(
    expr: &Spanned<NodeData>,
    definition_map: &mut HashMap<String, LangCompletionItem>,
    ident_offset: usize,
) -> bool {
    match &expr.0 {
        NodeData::Literal(_) => true,

        NodeData::Declaration(decl) => match decl {
            DeclarationNode::Variable(VariableNode {
                name,
                type_: _,
                value: _,
                mutable: _,
                span,
            }) => {
                definition_map.insert(name.clone(), LangCompletionItem::Variable(name.clone()));

                match get_completion_of(
                    &(
                        NodeData::Symbol(SymbolNode {
                            span: span.clone(),
                            value: name.clone(),
                        }),
                        span.clone(),
                    ),
                    definition_map,
                    ident_offset,
                ) {
                    true => get_completion_of(
                        &(
                            NodeData::Symbol(SymbolNode {
                                span: span.clone(),
                                value: name.clone(),
                            }),
                            span.clone(),
                        ),
                        definition_map,
                        ident_offset,
                    ),
                    false => false,
                }
            }

            _ => false,
        },

        NodeData::Expr(expr) => match expr {
            ExpressionNode::Binary(BinaryExpr {
                lhs,
                operator: _,
                rhs,
                span: _,
            }) => match get_completion_of(
                &(Box::into_inner(lhs.data.clone()), lhs.span.clone()),
                definition_map,
                ident_offset,
            ) {
                true => get_completion_of(
                    &(Box::into_inner(rhs.data.clone()), rhs.span.clone()),
                    definition_map,
                    ident_offset,
                ),
                false => false,
            },

            _ => false,
        },

        NodeData::Statement(stmt) => match stmt {
            StatementNode::Call(CallNode { func, args, span }) => {
                match get_completion_of(
                    &(
                        NodeData::Symbol(SymbolNode {
                            span: span.clone(),
                            value: func.clone(),
                        }),
                        span.clone(),
                    ),
                    definition_map,
                    ident_offset,
                ) {
                    true => {}
                    false => return false,
                }

                for expr in args {
                    match get_completion_of(
                        &(Box::into_inner(expr.value.data.clone()), expr.span.clone()),
                        definition_map,
                        ident_offset,
                    ) {
                        true => continue,
                        false => return false,
                    }
                }

                true
            }

            StatementNode::Condition(ConditionalNode {
                condition,
                block,
                else_block,
                span: _,
            }) => {
                match get_completion_of(
                    &(
                        Box::into_inner(condition.data.clone()),
                        condition.span.clone(),
                    ),
                    definition_map,
                    ident_offset,
                ) {
                    true => {}
                    false => return false,
                }

                match get_completion_of(
                    &(NodeData::Block(block.clone()), block.span.clone()),
                    definition_map,
                    ident_offset,
                ) {
                    true => {}
                    false => return false,
                }

                if let Some(else_block) = else_block {
                    get_completion_of(
                        &(NodeData::Block(else_block.clone()), else_block.span.clone()),
                        definition_map,
                        ident_offset,
                    )
                } else {
                    false
                }
            }

            StatementNode::Return(ReturnNode { span: _, value }) => {
                if let Some(value) = value {
                    get_completion_of(
                        &(Box::into_inner(value.data.clone()), value.span.clone()),
                        definition_map,
                        ident_offset,
                    )
                } else {
                    false
                }
            }
        },

        _ => false,
    }
}
