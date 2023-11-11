use std::collections::HashMap;

use anyhow::Result;
use cranelift_codegen::{entity::EntityRef, ir::AbiParam};
use cranelift_frontend::{FunctionBuilder, Variable};
use cranelift_module::Module;

use crate::{
    ast::expr::{Definition, Expression},
    translator::func::FunctionTranslator,
    types::{clif::IntoClifType, Type},
};

use super::backend::CraneliftBackend;

impl<T> CraneliftBackend<T>
where
    T: Module,
{
    pub fn translate_fn(
        &mut self,
        args: Vec<Box<Definition>>,
        ret: Type,
        content: Vec<Box<Expression>>,
        is_entry: bool,
    ) -> Result<()> {
        let mut args_map = HashMap::new();

        for arg in args.clone() {
            if let Definition::Argument(name, ty) = *arg {
                let idx = self.ctx.func.signature.params.len();

                self.ctx
                    .func
                    .signature
                    .params
                    .push(AbiParam::new(ty.into_clif_type().unwrap()));

                args_map.insert(name, idx);
            }
        }

        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(ret.into_clif_type().unwrap()));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_ctx);
        let entry = builder.create_block();

        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        let mut vars = HashMap::new();
        let mut var_idx = 0;

        for (idx, arg) in args.iter().enumerate() {
            if let Definition::Argument(name, ty) = *arg.clone() {
                let val = builder.block_params(entry)[idx];
                let var = Variable::new(var_idx);

                builder.declare_var(var, ty.into_clif_type().unwrap());
                builder.def_var(var, val);
                vars.insert(name, var);

                var_idx += 1;
            }
        }

        let mut translator = FunctionTranslator::new(
            builder,
            &mut self.module,
            &mut self.data_desc,
            ret,
            vars,
            var_idx,
            is_entry,
        );

        for expr in content {
            translator.translate(*expr)?;
        }

        translator.builder.finalize();

        Ok(())
    }
}
