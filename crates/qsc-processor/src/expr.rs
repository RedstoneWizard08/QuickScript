use qsc_ast::ast::{expr::ExpressionNode, node::data::NodeData};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_expr(
        &mut self,
        ctx: &mut ProcessorContext,
        mut expr: ExpressionNode,
    ) -> Result<NodeData> {
        match &mut expr {
            ExpressionNode::Binary(binary) => {
                binary.lhs = self.process_node(ctx, binary.lhs.clone())?;
                binary.rhs = self.process_node(ctx, binary.rhs.clone())?;
            }

            ExpressionNode::Unary(unary) => {
                unary.value = self.process_node(ctx, unary.value.clone())?;
            }
        };

        Ok(NodeData::Expr(expr))
    }
}
