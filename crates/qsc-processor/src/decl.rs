use qsc_ast::ast::{
    decl::DeclarationNode,
    node::{data::NodeData, ty::TypeNode},
};
use qsc_core::conv::IntoSourceSpan;

use crate::{ctx::ProcessorContext, error::ProcessingError, Processor, Result};

impl Processor {
    pub fn process_decl(
        &mut self,
        ctx: &mut ProcessorContext,
        mut decl: DeclarationNode,
    ) -> Result<NodeData> {
        match &mut decl {
            DeclarationNode::Function(func) => {
                ctx.func = Some(func.clone());

                for item in &mut func.content.data {
                    *item = self.process_node(ctx, item.clone())?;
                }

                if func.ret.is_none() {
                    func.ret = Some(TypeNode {
                        generics: Vec::new(),
                        name: "void".to_string(),
                        span: func.span.clone(),
                    })
                }
            }

            DeclarationNode::Global(_global) => todo!(),

            DeclarationNode::Variable(var) => {
                if let Some(ty) = &mut var.type_ {
                    *ty = self.process_type(ctx, ty.clone())?.as_type().unwrap();

                    if let Some(val) = &var.value {
                        if &val.data.as_type().unwrap() != ty {
                            let err = ProcessingError {
                                src: ctx.tree.src.clone(),
                                location: var.span.into_source_span(),
                                error: miette!("Declared type does not match value type!"),
                            };

                            std::result::Result::<(), _>::Err(err).unwrap();
                        }
                    }
                } else {
                    if let Some(val) = &var.value {
                        if let Ok(ty) = val.data.as_type() {
                            var.type_ = Some(ty);
                        }
                    }
                }
            }
        };

        Ok(NodeData::Declaration(decl))
    }
}
