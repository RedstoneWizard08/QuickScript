//! The initial Abstract Syntax Tree.
//! This is the container for the raw data that comes from pest's parsing.

pub mod decl;
pub mod expr;
pub mod literal;
pub mod node;
pub mod stmt;

use std::collections::HashMap;

use crate::{compat::WrappedNamedSource, span::StaticSpan};

use self::{
    decl::{func::FunctionNode, global::GlobalVariable},
    node::Node,
};

use miette::NamedSource;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractTree {
    #[serde(skip)]
    pub span: StaticSpan,
    #[serde(skip)]
    pub src: WrappedNamedSource<String>,
    #[serde(skip)]
    pub source: String,
    pub data: Vec<Node>,
}

impl AbstractTree {
    pub fn new(name: impl AsRef<str>, source: impl AsRef<str>) -> Self {
        let src = source.as_ref().to_string();

        Self {
            src: NamedSource::new(name, src.clone()).into(),
            span: StaticSpan::new(src.clone(), 0, src.len()),
            source: src,
            data: Vec::new(),
        }
    }

    pub fn functions(&self) -> HashMap<String, FunctionNode> {
        let mut funcs = HashMap::new();

        for node in &self.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(func) = decl.as_function() {
                    funcs.insert(func.name.to_string(), func);
                }
            }
        }

        funcs
    }

    pub fn globals(&self) -> HashMap<String, GlobalVariable> {
        let mut globals = HashMap::new();

        for node in &self.data {
            if let Ok(decl) = node.data.as_decl() {
                if let Ok(global) = decl.as_global() {
                    globals.insert(global.name.clone(), global);
                }
            }
        }

        globals
    }

    // TODO: `use` statements and actually do this
    pub fn imported_functions(&self) -> &[&str] {
        &["printf", "puts"]
    }

    // TODO: add support for custom structs and types
    pub fn types(&self) -> &[&str] {
        &[
            "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "char",
            "str",
        ]
    }
}
