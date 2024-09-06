use qsc_ast::ast::{expr::ExpressionNode, node::data::NodeData};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_expr(
        &self,
        ctx: &mut ProcessorContext,
        mut expr: ExpressionNode,
    ) -> Result<NodeData> {
        match &mut expr {
            ExpressionNode::Binary(binary) => {
                binary.lhs = self.process_node(ctx, &mut binary.lhs)?;
                binary.rhs = self.process_node(ctx, &mut binary.rhs)?;
            }

            ExpressionNode::Unary(unary) => {
                unary.value = self.process_node(ctx, &mut unary.value)?;
            }
        };

        Ok(NodeData::Expr(expr))
    }
}
