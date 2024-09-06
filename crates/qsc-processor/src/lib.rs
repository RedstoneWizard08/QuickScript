#![feature(box_into_inner)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate miette;

pub mod block;
pub mod ctx;
pub mod decl;
pub mod expr;
pub mod stmt;
pub mod sym;
pub mod ty;

use ctx::ProcessorContext;
use qsc_ast::ast::{
    node::{data::NodeData, Node},
    AbstractTree,
};
use qsc_core::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct Processor {
    pub ast: AbstractTree,
}

impl Processor {
    pub fn new(ast: AbstractTree) -> Self {
        Self { ast }
    }

    pub fn process(&mut self) -> Result<AbstractTree> {
        let mut ast = self.ast.clone();
        let ptr = &mut ast;
        let ptr2: &mut AbstractTree;

        unsafe {
            ptr2 = &mut *(ptr as *mut AbstractTree);
        }

        let mut ctx = ProcessorContext::new(ptr);

        for node in &mut ptr2.data {
            *node = self.process_node(&mut ctx, node)?;
        }

        Ok(ast)
    }

    pub fn process_node(&self, ctx: &mut ProcessorContext<'_>, node: &mut Node) -> Result<Node> {
        let data = match Box::into_inner(node.data.clone()) {
            NodeData::Block(block) => self.process_block(ctx, block),
            NodeData::Declaration(decl) => self.process_decl(ctx, decl),
            NodeData::Expr(expr) => self.process_expr(ctx, expr),
            NodeData::Literal(lit) => Ok(NodeData::Literal(lit)),
            NodeData::Statement(stmt) => self.process_stmt(ctx, stmt),
            NodeData::Symbol(sym) => self.process_symbol(ctx, sym),
            NodeData::Type(ty) => self.process_type(ctx, ty),
            NodeData::EOI => Ok(NodeData::EOI),
        }?;

        node.data = Box::new(data);

        Ok(node.clone())
    }
}
