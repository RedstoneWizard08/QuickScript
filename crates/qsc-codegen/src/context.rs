use std::collections::HashMap;

use cranelift_codegen::ir::Value;
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::{DataDescription, DataId, Module};

use qsc_ast::ast::{decl::func::FunctionNode, node::ty::TypeNode};

pub struct CompilerContext<'i, 'a, M: Module> {
    pub module: &'a mut M,
    pub data_desc: &'a mut DataDescription,
    pub functions: &'a mut HashMap<String, FunctionNode<'i>>,
    pub globals: &'a mut HashMap<String, DataId>,
    pub code: &'a mut Vec<(String, *const u8)>,
}

pub struct CodegenContext<'i, 'a> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode<'i>>)>,
    pub values: HashMap<String, (Value, TypeNode<'i>)>,
    pub builder: FunctionBuilder<'a>,
    pub ret: Option<TypeNode<'i>>,
    pub func: FunctionNode<'i>,
}
