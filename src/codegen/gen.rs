use anyhow::Result;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
use std::slice;

use crate::keyword::{AnyKeyword, FunctionKeyword};
use crate::token::Token;

pub struct Types {
    pub t_i8: Type,
    pub t_i16: Type,
    pub t_i32: Type,
    pub t_i64: Type,
    pub t_f32: Type,
    pub t_f64: Type,
    pub t_str: Type,
    pub t_char: Type,
    pub t_void: Type,
}

impl Types {
    pub fn new() -> Self {
        Self {
            t_i8: Type::int(8).unwrap(),
            t_i16: Type::int(16).unwrap(),
            t_i32: Type::int(32).unwrap(),
            t_i64: Type::int(64).unwrap(),
            t_f32: types::F32,
            t_f64: types::F64,
            t_str: types::I8X2XN,
            t_char: types::I8,
            t_void: types::I8,
        }
    }

    pub fn get_type(&self, a_type: Option<String>) -> Type {
        match a_type {
            Some(t) => match t.as_str() {
                "i8" => self.t_i8,
                "i16" => self.t_i16,
                "i32" => self.t_i32,
                "i64" => self.t_i64,
                "f32" => self.t_f32,
                "f64" => self.t_f64,
                "char" => self.t_char,
                "str" => self.t_str,

                _ => self.t_void,
            },

            None => self.t_void,
        }
    }

    pub fn get_type_name(&self, t: String) -> Type {
        match t.as_str() {
            "INTEGER" => self.t_i64,
            "FLOAT" => self.t_f64,
            "CHAR" => self.t_char,
            "STRING" => self.t_str,

            _ => self.t_void,
        }
    }
}

pub struct Compiler {
    types: Types,
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_description: DataDescription,
    module: JITModule,
}

impl Compiler {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();

        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);

        Self {
            types: Types::new(),
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    pub fn compile(&mut self, input: Vec<AnyKeyword>) -> Result<&[u8]> {
        for item in input {
            match item {
                AnyKeyword::Function(func) => self.compile_function(func)?,
                _ => {
                    return Err(anyhow!(
                        "Static members that are not functions are implemented yet!"
                    ))
                }
            };
        }

        Ok(self.ctx.compiled_code().unwrap().buffer.data())
    }

    pub fn compile_function(&mut self, input: FunctionKeyword) -> Result<*const u8> {
        let (name, args, ret, block) = input.value.unwrap();

        self.translate(args, ret, block)?;

        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.ctx.func.signature)?;

        self.module.define_function(id, &mut self.ctx)?;

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);

        Ok(code)
    }

    pub fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8]> {
        self.data_description.define(contents.into_boxed_slice());

        let id = self
            .module
            .declare_data(name, Linkage::Export, true, false)?;

        self.module.define_data(id, &self.data_description)?;

        self.data_description.clear();
        self.module.finalize_definitions().unwrap();

        let buffer = self.module.get_finalized_data(id);

        Ok(unsafe { slice::from_raw_parts(buffer.0, buffer.1) })
    }

    fn translate(
        &mut self,
        args: Vec<(Token, Token)>,
        ret: Option<Token>,
        block: Vec<AnyKeyword>,
    ) -> Result<()> {
        for (_, arg_type) in args {
            self.ctx
                .func
                .signature
                .params
                .push(AbiParam::new(self.types.get_type(arg_type.value)));
        }

        if let Some(ret_type) = ret.clone() {
            self.ctx
                .func
                .signature
                .returns
                .push(AbiParam::new(self.types.get_type(ret_type.value)));
        }

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();

        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut trans = FunctionTranslator {
            types: Types::new(),
            builder,
            var: VariableBuilder::new(),
            module: &mut self.module,
            vars: HashMap::new(),
            ret: ret.map(|v| self.types.get_type(v.value)),
        };

        for kw in block {
            trans.translate_kw(kw);
        }

        let return_variable = trans.vars.get("__return__").unwrap();
        let return_value = trans.builder.use_var(*return_variable);

        trans.builder.ins().return_(&[return_value]);
        trans.builder.finalize();

        Ok(())
    }
}

struct FunctionTranslator<'a> {
    types: Types,
    builder: FunctionBuilder<'a>,
    var: VariableBuilder,
    module: &'a mut JITModule,
    vars: HashMap<String, Variable>,
    ret: Option<Type>,
}

impl<'a> FunctionTranslator<'a> {
    /// When you write out instructions in Cranelift, you get back `Value`s. You
    /// can then use these references in other instructions.
    fn translate_kw(&mut self, kw: AnyKeyword) -> Value {
        match kw {
            AnyKeyword::Variable(kw) => {
                let (name, type_, val) = kw.value.unwrap();
                let val = self.translate_value(val);
                let type_ = self.types.get_type(Some(type_));
                let var = self.var.create_var(&mut self.builder, type_, val);

                self.vars.insert(name, var);
                self.builder.use_var(var)
            }

            AnyKeyword::Return(kw) => {
                let tkn = kw.value.unwrap().0;
                let val = self.translate_value(vec![tkn]);

                let var = self.var.create_var(
                    &mut self.builder,
                    self.ret.unwrap(),
                    val,
                );

                self.vars.insert(String::from("__return__"), var);
                self.builder.use_var(var)
            }

            AnyKeyword::Print(kw) => {
                let args = kw.value.unwrap().args;

                self.translate_call(String::from("print"), args)
            }

            v => {
                println!("Cannot process: {:?}", v);

                self.builder.ins().null(self.types.t_void)
            },
        }
    }

    fn translate_value(&mut self, tkns: Vec<Token>) -> Value {
        let tkn = tkns[0].clone();
        let name = tkn.name;

        match name.as_str() {
            "STRING" => {
                let val = tkn.value.unwrap();
                let handle = self
                    .builder
                    .func
                    .dfg
                    .constants
                    .insert(val.as_bytes().to_vec().into());

                self.builder.ins().vconst(self.types.t_str, handle)
            }

            "CHAR" => {
                let val = tkn.value.unwrap().chars().nth(0).unwrap() as i8;

                self.builder.ins().iconst(self.types.t_i8, i64::from(val))
            }

            "INTEGER" => {
                let val = tkn.value.unwrap().parse::<i64>().unwrap();

                self.builder.ins().iconst(self.types.t_i64, val)
            }

            "FLOAT" => {
                let val = tkn.value.unwrap().parse::<f32>().unwrap();

                self.builder.ins().f32const(val)
            }

            "IDENT" => {
                let val = tkn.value.unwrap();

                if self.vars.contains_key(&val) {
                    return self.builder.use_var(*self.vars.get(&val).unwrap());
                }

                self.builder.ins().null(self.types.t_void)
            }

            _ => todo!(),
        }
    }

    fn translate_call(&mut self, name: String, args: Vec<Token>) -> Value {
        let mut sig = self.module.make_signature();

        for arg in args.clone() {
            sig.params
                .push(AbiParam::new(self.types.get_type_name(arg.name)));
        }

        sig.returns.push(AbiParam::new(self.types.t_void));

        let callee = self
            .module
            .declare_function(&name, Linkage::Import, &sig)
            .expect("problem declaring function");

        let local_callee = self.module.declare_func_in_func(callee, self.builder.func);
        let mut arg_values = Vec::new();

        for arg in args {
            arg_values.push(self.translate_value(vec![arg]))
        }

        let call = self.builder.ins().call(local_callee, &arg_values);

        self.builder.inst_results(call)[0]
    }
}

pub struct VariableBuilder {
    index: usize,
}

impl VariableBuilder {
    fn new() -> Self {
        Self { index: 0 }
    }

    fn create_var(&mut self, builder: &mut FunctionBuilder, t: Type, value: Value) -> Variable {
        let variable = Variable::new(self.index);
        builder.declare_var(variable, t);
        self.index += 1;
        builder.def_var(variable, value);
        variable
    }
}
