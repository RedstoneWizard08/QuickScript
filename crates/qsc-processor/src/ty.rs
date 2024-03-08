use qsc_ast::ast::node::{data::NodeData, ty::TypeNode};
use qsc_core::conv::IntoSourceSpan;

use crate::{ctx::ProcessorContext, error::ProcessingError, Processor, Result};

impl Processor {
    pub fn process_type(&mut self, ctx: &mut ProcessorContext, ty: TypeNode) -> Result<NodeData> {
        if !ctx.tree.types().contains(&ty.as_str().as_str()) {
            return Err(ProcessingError {
                src: ctx.tree.src.clone(),
                location: ty.span.into_source_span(),
                error: miette!("Unresolved type: {}", ty.as_str()),
            }
            .into());
        }

        Ok(NodeData::Type(ty))
    }
}
