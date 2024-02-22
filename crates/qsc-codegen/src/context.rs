use std::collections::HashMap;

use cranelift_codegen::ir::Value;
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::{DataDescription, DataId, Module};

use qsc_ast::var::FunctionData;

pub struct CompilerContext<'a, M: Module> {
    pub module: &'a mut M,
    pub data_desc: &'a mut DataDescription,
    pub functions: &'a mut HashMap<String, FunctionData>,
    pub globals: &'a mut HashMap<String, DataId>,
    pub code: &'a mut Vec<(String, *const u8)>,
}

pub struct CodegenContext<'a> {
    pub locals: HashMap<String, DataId>,
    pub vars: HashMap<String, (Variable, String)>,
    pub values: HashMap<String, (Value, String)>,
    pub builder: FunctionBuilder<'a>,
    pub ret: String,
    pub func: FunctionData,
}