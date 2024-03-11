use qsc_ast::ast::{
    decl::DeclarationNode,
    node::{data::NodeData, ty::TypeNode},
};
use qsc_core::{conv::IntoSourceSpan, error::processor::ProcessorError};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_decl(
        &mut self,
        ctx: &mut ProcessorContext,
        mut decl: DeclarationNode,
    ) -> Result<NodeData> {
        match &mut decl {
            DeclarationNode::Function(func) => {
                ctx.func = Some(func.clone());
                func.content = self.process_block(ctx, func.content.clone())?.as_block()?;

                if func.ret.is_none() {
                    func.ret = Some(TypeNode {
                        generics: Vec::new(),
                        name: "void".to_string(),
                        span: func.span.clone(),
                    })
                }

                ctx.func = None;
            }

            DeclarationNode::Global(_global) => todo!(),

            DeclarationNode::Variable(var) => {
                if var.mutable && var.value.is_none() {
                    return Err(ProcessorError {
                        src: ctx.tree.src.clone().into(),
                        location: var.span.into_source_span(),
                        error: miette!("Uninitialized variables must be mutable!"),
                    }
                    .into());
                }

                if let Some(ty) = &mut var.type_ {
                    *ty = self.process_type(ctx, ty.clone())?.as_type().unwrap();

                    if let Some(val) = &var.value {
                        if val
                            .data
                            .get_type(&ctx.func.clone().map(|v| v.name), &ctx.tree)?
                            != ty.as_str()
                        {
                            return Err(ProcessorError {
                                src: ctx.tree.src.clone().into(),
                                location: var.span.into_source_span(),
                                error: miette!("Declared type does not match value type!"),
                            }
                            .into());
                        }
                    }
                } else {
                    if let Some(val) = &var.value {
                        if let Ok(ty) = val
                            .data
                            .get_type(&ctx.func.clone().map(|v| v.name), &ctx.tree)
                        {
                            var.type_ = Some(TypeNode {
                                generics: Vec::new(),
                                name: ty,
                                span: val.span.clone(),
                            });

                            debug!(
                                "Changed {}'s type to: {}",
                                var.name,
                                var.type_.clone().unwrap().as_str()
                            );
                        } else {
                            return Err(ProcessorError {
                                src: ctx.tree.src.clone().into(),
                                location: var.span.into_source_span(),
                                error: miette!("Cannot infer type based on value!"),
                            }
                            .into());
                        }
                    } else {
                        return Err(ProcessorError {
                            src: ctx.tree.src.clone().into(),
                            location: var.span.into_source_span(),
                            error: miette!(
                                "An explicit type must be specified if there is no value!"
                            ),
                        }
                        .into());
                    }
                }
            }
        };

        Ok(NodeData::Declaration(decl))
    }
}
