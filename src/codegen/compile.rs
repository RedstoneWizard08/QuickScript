use anyhow::Result;
use cranelift_codegen::control::ControlPlane;
use cranelift_module::{Linkage, Module};

use crate::ast::expr::{Definition, Expression};

use super::{backend::CraneliftBackend, jit::JITFinish};

impl<T> CraneliftBackend<T>
where
    T: Module + 'static,
{
    pub fn compile(&mut self, exprs: Vec<Expression>) -> Result<()> {
        // At the top level, only functions can be declared atm

        for expr in exprs {
            match expr {
                Expression::Define(def) => match def {
                    Definition::Function(mut name, args, ret, content) => {
                        if name == "main" {
                            name = String::from("_start");
                        }

                        self.ctx.set_disasm(self.disasm);
                        self.translate_fn(args, ret, content, name == "_start")?;

                        let id = self.module.declare_function(
                            &name,
                            Linkage::Export,
                            &self.ctx.func.signature,
                        )?;

                        self.module.define_function(id, &mut self.ctx)?;
                        self.fns.push(self.ctx.func.clone());

                        self.code.push(
                            self.ctx
                                .compile(self.module.isa(), &mut ControlPlane::default())
                                .unwrap()
                                .clone(),
                        );

                        self.module.clear_context(&mut self.ctx);
                        self.jit_finish_fn(name, id)?;
                    }

                    _ => (),
                },

                _ => (),
            }
        }

        Ok(())
    }
}
