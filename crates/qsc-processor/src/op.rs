use qsc_ast::{expr::Expr, op::Operation};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_operation(
        &mut self,
        ctx: &mut ProcessorContext,
        mut op: Operation,
    ) -> Result<Expr> {
        match &mut op {
            Operation::Binary(binary) => {
                binary.left = self.process_expr(ctx, binary.left.clone())?;
                binary.right = self.process_expr(ctx, binary.right.clone())?;
            }

            Operation::Unary(unary) => {
                unary.value = self.process_expr(ctx, unary.value.clone())?;
            }
        };

        Ok(Expr::Operation(op))
    }
}
