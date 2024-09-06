#![feature(box_into_inner)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate miette;

pub mod block;
pub mod ctx;
pub mod decl;
pub mod op;
pub mod stmt;
pub mod sym;
pub mod ty;

use ctx::ProcessorContext;
use qsc_ast::{ast::AbstractTree, expr::Expr};
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
        let mut ctx = ProcessorContext::new(ast.clone());

        for node in &mut ast.data {
            *node = self.process_expr(&mut ctx, node.clone())?;
        }

        Ok(ast)
    }

    pub fn process_expr(&mut self, ctx: &mut ProcessorContext, expr: Expr) -> Result<Expr> {
        let data = match expr {
            Expr::Literal(lit) => Ok(Expr::Literal(lit)),
            Expr::Name(name) => self.process_name(name),
            Expr::Block(block) => self.process_block(ctx, block),
            Expr::Variable(var) => self.process_decl(ctx, var),
            Expr::Operation(op) => self.process_operation(ctx, op),
            Expr::Call(call) => self.process_call(ctx, call),
            Expr::Return(ret) => self.process_return(ctx, ret),
            Expr::Conditional(cond) => self.process_cond(ctx, cond),
            Expr::Function(func) => self.process_func(ctx, func),
            Expr::Type(ty) => self.process_type(ctx, ty),
        }?;

        Ok(expr)
    }
}
