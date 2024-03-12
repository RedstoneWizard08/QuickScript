use im_rc::Vector;
use qsc_ast::ast::{
    decl::{var::VariableNode, DeclarationNode},
    expr::{binary::BinaryExpr, unary::UnaryExpr, ExpressionNode},
    node::{data::NodeData, sym::SymbolNode},
    stmt::{call::CallNode, cond::ConditionalNode, StatementNode},
    AbstractTree,
};

use crate::util::Spanned;

#[derive(Debug, Clone)]
pub enum ReferenceSymbol {
    Founded(Spanned<String>),
    Founding(usize),
}

pub fn get_reference(
    ast: &AbstractTree,
    ident_offset: usize,
    include_self: bool,
) -> Vec<Spanned<String>> {
    let mut vector = Vector::new();
    let mut reference_list = vec![];

    // for (_, v) in ast.iter() {
    //     if v.name.1.end < ident_offset {
    //         vector.push_back(v.name.clone());
    //     }
    // }

    let funcs = ast.functions();
    let mut kv_list = funcs.iter().collect::<Vec<_>>();

    kv_list.sort_by(|a, b| a.1.span.start.cmp(&b.1.span.start));

    let mut reference_symbol = ReferenceSymbol::Founding(ident_offset);
    // let mut fn_vector = Vector::new();

    for (_, v) in kv_list {
        let span = v.span.clone();

        if ident_offset >= span.start && ident_offset < span.end {
            reference_symbol = ReferenceSymbol::Founded((v.name.clone(), span));

            if include_self {
                reference_list.push((v.name.clone(), v.span.clone()));
            }
        };

        vector.push_back((v.name.clone(), v.span.clone()));

        let args = v
            .args
            .iter()
            .map(|arg| {
                if ident_offset >= arg.span.start && ident_offset < arg.span.end {
                    reference_symbol =
                        ReferenceSymbol::Founded((arg.name.clone(), arg.span.clone()));

                    if include_self {
                        reference_list.push((arg.name.clone(), arg.span.clone()));
                    }
                }

                (arg.name.clone(), arg.span.clone())
            })
            .collect::<Vector<_>>();

        get_reference_of_expr(
            ast,
            &(NodeData::Block(v.content.clone()), v.content.span.clone()),
            args + vector.clone(),
            reference_symbol.clone(),
            &mut reference_list,
            include_self,
        );
    }

    reference_list
}

pub fn get_reference_of_expr(
    ast: &AbstractTree,
    expr: &Spanned<NodeData>,
    definition_ass_list: Vector<Spanned<String>>,
    reference_symbol: ReferenceSymbol,
    reference_list: &mut Vec<Spanned<String>>,
    include_self: bool,
) {
    match &expr.0 {
        NodeData::Literal(_) => {}

        NodeData::Symbol(SymbolNode { value, span }) => {
            if let ReferenceSymbol::Founded((symbol_name, symbol_span)) = reference_symbol {
                if &symbol_name == value {
                    let index = definition_ass_list
                        .iter()
                        .position(|decl| decl.0 == symbol_name);
                    if let Some(symbol) = index.map(|i| definition_ass_list.get(i).unwrap().clone())
                    {
                        if symbol == (symbol_name, symbol_span) {
                            reference_list.push((value.clone(), span.clone()));
                        }
                    };
                }
            }

            // if ident_offset >= local.1.start && ident_offset < local.1.end {
            //     let index = definition_ass_list
            //         .iter()
            //         .position(|decl| decl.0 == local.0);
            //     (
            //         false,
            //         index.map(|i| definition_ass_list.get(i).unwrap().clone()),
            //     )
            // } else {
            //     (true, None)
            // }
        }

        NodeData::Declaration(decl) => match decl {
            DeclarationNode::Variable(VariableNode {
                name,
                type_: _,
                value,
                mutable: _,
                span,
            }) => {
                let new_decl = Vector::unit((name.clone(), span.clone()));

                let next_symbol = match reference_symbol {
                    ReferenceSymbol::Founding(ident) if ident >= span.start && ident < span.end => {
                        let spanned_name = (name.clone(), span.clone());

                        if include_self {
                            reference_list.push(spanned_name.clone());
                        }

                        ReferenceSymbol::Founded(spanned_name)
                    }
                    _ => reference_symbol,
                };

                get_reference_of_expr(
                    ast,
                    &(
                        NodeData::Symbol(SymbolNode {
                            span: span.clone(),
                            value: name.clone(),
                        }),
                        span.clone(),
                    ),
                    definition_ass_list.clone(),
                    next_symbol.clone(),
                    reference_list,
                    include_self,
                );

                if let Some(value) = value {
                    get_reference_of_expr(
                        ast,
                        &(Box::into_inner(value.data.clone()), value.span.clone()),
                        new_decl + definition_ass_list,
                        next_symbol,
                        reference_list,
                        include_self,
                    );
                }
            }

            _ => {}
        },

        NodeData::Expr(expr) => match expr {
            ExpressionNode::Binary(BinaryExpr {
                lhs,
                operator: _,
                rhs,
                span: _,
            }) => {
                get_reference_of_expr(
                    ast,
                    &(Box::into_inner(lhs.data.clone()), lhs.span.clone()),
                    definition_ass_list.clone(),
                    reference_symbol.clone(),
                    reference_list,
                    include_self,
                );

                get_reference_of_expr(
                    ast,
                    &(Box::into_inner(rhs.data.clone()), rhs.span.clone()),
                    definition_ass_list,
                    reference_symbol,
                    reference_list,
                    include_self,
                );
            }

            ExpressionNode::Unary(UnaryExpr {
                value,
                negative: _,
                span: _,
            }) => get_reference_of_expr(
                ast,
                &(Box::into_inner(value.data.clone()), value.span.clone()),
                definition_ass_list.clone(),
                reference_symbol.clone(),
                reference_list,
                include_self,
            ),
        },

        NodeData::Statement(expr) => match expr {
            StatementNode::Call(CallNode { func, args, span }) => {
                get_reference_of_expr(
                    ast,
                    &(
                        NodeData::Symbol(SymbolNode {
                            span: span.clone(),
                            value: func.clone(),
                        }),
                        span.clone(),
                    ),
                    definition_ass_list.clone(),
                    reference_symbol.clone(),
                    reference_list,
                    include_self,
                );

                for expr in args {
                    get_reference_of_expr(
                        ast,
                        &(Box::into_inner(expr.value.data.clone()), expr.span.clone()),
                        definition_ass_list.clone(),
                        reference_symbol.clone(),
                        reference_list,
                        include_self,
                    );
                }
            }

            StatementNode::Condition(ConditionalNode {
                condition,
                block,
                else_block,
                span: _,
            }) => {
                get_reference_of_expr(
                    ast,
                    &(
                        Box::into_inner(condition.data.clone()),
                        condition.span.clone(),
                    ),
                    definition_ass_list.clone(),
                    reference_symbol.clone(),
                    reference_list,
                    include_self,
                );

                get_reference_of_expr(
                    ast,
                    &(NodeData::Block(block.clone()), block.span.clone()),
                    definition_ass_list.clone(),
                    reference_symbol.clone(),
                    reference_list,
                    include_self,
                );

                if let Some(else_block) = else_block {
                    get_reference_of_expr(
                        ast,
                        &(NodeData::Block(else_block.clone()), else_block.span.clone()),
                        definition_ass_list,
                        reference_symbol.clone(),
                        reference_list,
                        include_self,
                    );
                }
            }

            _ => {}
        },

        _ => {}
    }
}
