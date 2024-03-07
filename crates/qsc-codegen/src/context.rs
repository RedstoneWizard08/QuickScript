use std::{collections::HashMap, sync::Arc};

use cranelift_codegen::{
    ir::{Function, Value},
    CompiledCode, Context,
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, Module};

use parking_lot::RwLock;
use qsc_ast::ast::{decl::func::FunctionNode, node::ty::TypeNode};

pub struct CodegenContext<'a> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, Option<TypeNode<'a>>)>,
    pub values: HashMap<String, (Value, TypeNode<'a>)>,
    pub builder: Arc<RwLock<FunctionBuilder<'a>>>,
    pub ret: Option<TypeNode<'a>>,
    pub func: FunctionNode<'a>,
}

pub struct CompilerContext<'a, M: Module> {
    pub builder_ctx: FunctionBuilderContext,
    pub ctx: Context,
    pub data_desc: DataDescription,
    pub module: M,
    pub functions: HashMap<String, FunctionNode<'a>>,
    pub globals: HashMap<String, DataId>,
    pub fns: Vec<Function>,
    pub vcode: Vec<CompiledCode>,
    pub code: Vec<(String, *const u8)>,
}
