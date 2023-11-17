use std::collections::HashMap;

use anyhow::Result;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{types, AbiParam, InstBuilder, Value},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::{DataDescription, Linkage, Module};

use crate::{
    ast::expr::{Expression, Definition},
    codegen::data::create_data,
    types::{clif::IntoClifType, Type},
    util::random_string,
};

pub struct FunctionTranslator<'a, T>
where
    T: Module,
{
    pub builder: FunctionBuilder<'a>,
    pub variables: HashMap<String, Variable>,
    pub module: &'a mut T,
    pub data_desc: &'a mut DataDescription,
    pub var_idx: usize,
    pub ret_type: Type,
    pub is_entry: bool,
}

impl<'a, T> FunctionTranslator<'a, T>
where
    T: Module,
{
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a mut T,
        data_desc: &'a mut DataDescription,
        ret_type: Type,
        variables: HashMap<String, Variable>,
        var_idx: usize,
        is_entry: bool,
    ) -> Self {
        Self {
            builder,
            module,
            variables,
            data_desc,
            ret_type,
            var_idx,
            is_entry,
        }
    }

    pub fn create_var(&mut self, name: &str, ty: Type) -> Variable {
        let var = Variable::new(self.var_idx);

        self.var_idx += 1;
        self.variables.insert(name.to_string(), var);
        self.builder.declare_var(var, ty.into_clif_type().unwrap());

        var
    }

    pub fn null(&mut self) -> Value {
        Value::from_u32(0)
    }

    pub fn translate(&mut self, expr: Expression) -> Result<Value> {
        match expr {
            Expression::Return(val) => {
                let val = self.translate(*val)?;
                let var = self.create_var("__return__", self.ret_type.clone());

                self.builder.def_var(var, val);

                let val = self.builder.use_var(var);

                if self.is_entry {
                    let mut sig = self.module.make_signature();

                    sig.params.push(AbiParam::new(types::I32));

                    let callee = self
                        .module
                        .declare_function("exit", Linkage::Import, &sig)?;

                    let local_callee = self.module.declare_func_in_func(callee, self.builder.func);
                    let call = self.builder.ins().call(local_callee, &vec![val]);
                    let res = self.builder.inst_results(call);

                    if let Some(val) = res.get(0) {
                        return Ok(*val);
                    }
                }

                self.builder.ins().return_(&[val]);

                Ok(self.null())
            }

            Expression::Define(def) => match def {
                Definition::Variable(name, ty, val) => {
                    let var = self.create_var(&name, ty);
                    let val = self.translate(*val)?;

                    self.builder.def_var(var, val);

                    Ok(self.null())
                }

                _ => Ok(self.null())
            }

            Expression::String(val) => {
                let name = random_string(32);

                self.use_data(name, format!("{}\0", val))
            }

            // TODO: Also include non-f32s.
            Expression::Float(flt) => Ok(self.builder.ins().f32const(flt)),

            // TODO: Also include non-i32s.
            Expression::Number(num) => Ok(self.builder.ins().iconst(types::I32, i64::from(num))),

            Expression::Identifier(name) => {
                let var = self
                    .variables
                    .get(&name)
                    .ok_or(anyhow!("Cannot find variable with name {}!", name))?;

                Ok(self.builder.use_var(*var))
            }

            Expression::Call(name, args) => {
                let mut sig = self.module.make_signature();

                for arg in args.clone() {
                    // TODO: Better type guessing
                    sig.params.push(AbiParam::new(arg.get_type()));
                }

                // TODO: Returns

                let callee = self.module.declare_function(&name, Linkage::Import, &sig)?;

                let local_callee = self.module.declare_func_in_func(callee, self.builder.func);
                let mut arg_values = Vec::new();

                for arg in args {
                    arg_values.push(self.translate(*arg)?);
                }

                let call = self.builder.ins().call(local_callee, &arg_values);
                let res = self.builder.inst_results(call);

                if let Some(val) = res.get(0) {
                    return Ok(*val);
                }

                Ok(self.null())
            }

            _ => Ok(self.null()),
        }
    }

    pub fn use_data<V>(&mut self, name: String, val: V) -> Result<Value>
    where
        V: Into<Vec<u8>>,
    {
        create_data(&mut self.module, &mut self.data_desc, &name, val)?;

        let id = self
            .module
            .declare_data(&name, Linkage::Export, true, false)?;

        let local_id = self.module.declare_data_in_func(id, self.builder.func);
        let pointer = self.module.target_config().pointer_type();

        Ok(self.builder.ins().symbol_value(pointer, local_id))
    }
}
