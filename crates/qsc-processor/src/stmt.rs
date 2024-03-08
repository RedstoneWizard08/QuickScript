use qsc_ast::ast::{node::data::NodeData, stmt::StatementNode};
use qsc_core::conv::IntoSourceSpan;

use crate::{ctx::ProcessorContext, error::ProcessingError, Processor, Result};

impl Processor {
    pub fn process_stmt(
        &mut self,
        ctx: &mut ProcessorContext,
        mut stmt: StatementNode,
    ) -> Result<NodeData> {
        match &mut stmt {
            StatementNode::Call(call) => {
                if !self.ast.functions().contains_key(&call.func)
                    && !self.ast.imported_functions().contains(&call.func.as_str())
                {
                    let err = ProcessingError {
                        src: ctx.tree.src.clone(),
                        location: call.span.into_source_span(),
                        error: miette!("Cannot find function \"{}\"!", call.func),
                    };

                    std::result::Result::<(), _>::Err(err).unwrap();
                }

                for arg in &mut call.args {
                    arg.value = self.process_node(ctx, arg.value.clone())?;
                }
            }

            StatementNode::Return(ret) => {
                if let Some(val) = &mut ret.value {
                    *val = self.process_node(ctx, val.clone())?;
                }
            }
        };

        Ok(NodeData::Statement(stmt))
    }
}
