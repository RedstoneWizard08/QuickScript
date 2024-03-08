use qsc_ast::ast::node::{data::NodeData, sym::SymbolNode};
use qsc_core::conv::IntoSourceSpan;

use crate::{ctx::ProcessorContext, error::ProcessingError, Processor, Result};

impl Processor {
    pub fn process_symbol(
        &mut self,
        ctx: &mut ProcessorContext,
        sym: SymbolNode,
    ) -> Result<NodeData> {
        if let Some(func) = &ctx.func {
            if !func.variables().contains_key(&sym.value) {
                return Err(ProcessingError {
                    src: ctx.tree.src.clone(),
                    location: sym.span.into_source_span(),
                    error: miette!("Cannot find symbol: {}", sym.value),
                }
                .into());
            }
        }

        if !ctx.tree.globals().contains_key(&sym.value) {
            return Err(ProcessingError {
                src: ctx.tree.src.clone(),
                location: sym.span.into_source_span(),
                error: miette!("Cannot find symbol: {}", sym.value),
            }
            .into());
        }

        Ok(NodeData::Symbol(sym))
    }
}
