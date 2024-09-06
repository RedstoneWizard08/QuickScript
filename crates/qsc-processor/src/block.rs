use qsc_ast::ast::node::{block::Block, data::NodeData};

use crate::{ctx::ProcessorContext, Processor, Result};

impl Processor {
    pub fn process_block(&self, ctx: &mut ProcessorContext, mut block: Block) -> Result<NodeData> {
        for item in &mut block.data {
            *item = self.process_node(ctx, item)?;
        }

        Ok(NodeData::Block(block))
    }
}
