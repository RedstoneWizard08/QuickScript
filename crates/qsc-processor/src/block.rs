use qsc_ast::{block::Block, expr::Expr};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_block(
        &mut self,
        ctx: &mut ProcessorContext,
        mut block: Block,
    ) -> Result<Expr> {
        for item in &mut block.body {
            *item = self.process_expr(ctx, item.clone())?;
        }

        Ok(Expr::Block(block))
    }
}
