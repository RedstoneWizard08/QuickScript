use qsc_ast::{call::Call, expr::Expr};
use qsc_core::{conv::IntoSourceSpan, error::processor::ProcessorError};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_call(&mut self, ctx: &mut ProcessorContext, mut call: Call) -> Result<Expr> {
        if !self.ast.functions().contains_key(&call.func)
            && !self.ast.imported_functions().contains(&call.func.as_str())
            && !self.ast.externs().contains_key(&call.func)
        {
            return Err(ProcessorError {
                src: ctx.tree.src.clone().into(),
                location: call.span.into_source_span(),
                error: miette!("Cannot find function \"{}\"!", call.func),
            }
            .into());
        }

        for arg in &mut call.args {
            arg.value = self.process_expr(ctx, arg.value.clone())?;
        }

        Ok(Expr::Call(call))
    }

    pub fn process_stmt(
        &mut self,
        ctx: &mut ProcessorContext,
        mut stmt: StatementNode,
    ) -> Result<NodeData> {
        match &mut stmt {
            StatementNode::Call(call) => {}

            StatementNode::Return(ret) => {
                if let Some(val) = &mut ret.value {
                    *val = self.process_expr(ctx, val.clone())?;
                }
            }

            StatementNode::Condition(cond) => {
                cond.condition = self.process_expr(ctx, cond.condition.clone())?;
                cond.block = self.process_block(ctx, cond.block.clone())?.as_block()?;
            }
        };

        Ok(NodeData::Statement(stmt))
    }
}
