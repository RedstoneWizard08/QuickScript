use qsc_ast::ast::node::{data::NodeData, ty::TypeNode};
use qsc_core::{conv::IntoSourceSpan, error::processor::ProcessorError};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_type(&self, ctx: &ProcessorContext, ty: TypeNode) -> Result<NodeData> {
        if !ctx.tree.types().contains(&ty.as_str().as_str()) {
            return Err(ProcessorError {
                src: ctx.tree.src.clone().into(),
                location: ty.span.into_source_span(),
                error: miette!("Unresolved type: {}", ty.as_str()),
            }
            .into());
        }

        Ok(NodeData::Type(ty))
    }
}
