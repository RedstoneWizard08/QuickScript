use qsc_ast::ast::node::{data::NodeData, sym::SymbolNode};
use qsc_core::{conv::IntoSourceSpan, error::processor::ProcessorError};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_symbol(
        &mut self,
        ctx: &mut ProcessorContext,
        sym: SymbolNode,
    ) -> Result<NodeData> {
        if let Some(func) = &ctx.func {
            if !func.variables().contains_key(&sym.value)
                && !ctx.tree.globals().contains_key(&sym.value)
            {
                return Err(ProcessorError {
                    src: ctx.tree.src.clone().into(),
                    location: sym.span.into_source_span(),
                    error: miette!("Cannot find symbol: {}", sym.value),
                }
                .into());
            }

            return Ok(NodeData::Symbol(sym));
        }

        if !ctx.tree.globals().contains_key(&sym.value) {
            return Err(ProcessorError {
                src: ctx.tree.src.clone().into(),
                location: sym.span.into_source_span(),
                error: miette!("Cannot find symbol: {}", sym.value),
            }
            .into());
        }

        Ok(NodeData::Symbol(sym))
    }
}
