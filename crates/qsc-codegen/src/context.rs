use std::{cell::RefCell, collections::HashMap, rc::Rc};

use cranelift_codegen::{ir::{Function, Value}, CompiledCode, Context};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::{DataDescription, DataId, Module};

use qsc_ast::ast::{decl::func::FunctionNode, node::ty::TypeNode};

pub struct CompilerContext<'a, M: Module> {
    pub module: &'a mut M,
    pub data_desc: &'a mut DataDescription,
    pub functions: &'a mut HashMap<String, FunctionNode<'a>>,
    pub globals: &'a mut HashMap<String, DataId>,
    pub code: &'a mut Vec<(String, *const u8)>,
    pub fns: &'a mut Vec<Function>,
    pub vcode: &'a mut Vec<CompiledCode>,
}

pub struct CodegenContext<'a> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode<'a>>)>,
    pub values: HashMap<String, (Value, TypeNode<'a>)>,
    pub builder: FunctionBuilder<'a>,
    pub ret: Option<TypeNode<'a>>,
    pub func: FunctionNode<'a>,
}
