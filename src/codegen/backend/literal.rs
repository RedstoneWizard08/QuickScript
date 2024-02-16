use anyhow::Result;
use cranelift_module::{DataId, Linkage};

use crate::{ast::literal::Literal, util::random_string};

use super::Backend;

pub trait LiteralCompiler<'a>: Backend<'a> {
    type O;

    fn compile_literal(&mut self, expr: Literal) -> Result<Self::O>;
    fn compile_none(&mut self) -> Result<Self::O>;
    fn compile_bool(&mut self, value: bool) -> Result<Self::O>;
    fn compile_int(&mut self, value: i64) -> Result<Self::O>;
    fn compile_float(&mut self, value: f64) -> Result<Self::O>;
    fn compile_string(&mut self, value: String) -> Result<Self::O>;
    fn compile_char(&mut self, value: char) -> Result<Self::O>;
}

impl<'a, T: Backend<'a>> LiteralCompiler<'a> for T {
    type O = DataId;

    fn compile_literal(&mut self, expr: Literal) -> Result<Self::O> {
        match expr {
            Literal::None => self.compile_none(),
            Literal::Boolean(b) => self.compile_bool(b),
            Literal::Integer(i) => self.compile_int(i),
            Literal::Float(f) => self.compile_float(f),
            Literal::String(s) => self.compile_string(s),
            Literal::Char(c) => self.compile_char(c),
        }
    }

    fn compile_none(&mut self) -> Result<Self::O> {
        self.data_desc().define(Box::new([0]));

        let name = format!("literal_none_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }

    fn compile_bool(&mut self, value: bool) -> Result<Self::O> {
        self.data_desc().define(Box::new([value as u8]));

        let name = format!("literal_bool_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }

    fn compile_int(&mut self, value: i64) -> Result<Self::O> {
        self.data_desc().define(Box::new(value.to_le_bytes()));

        let name = format!("literal_int_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }

    fn compile_float(&mut self, value: f64) -> Result<Self::O> {
        self.data_desc().define(Box::new(value.to_le_bytes()));

        let name = format!("literal_float_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }

    fn compile_string(&mut self, value: String) -> Result<Self::O> {
        self.data_desc()
            .define(value.into_bytes().into_boxed_slice());

        let name = format!("literal_string_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }

    fn compile_char(&mut self, value: char) -> Result<Self::O> {
        self.data_desc().define(Box::new([value as u8]));

        let name = format!("literal_char_{}", random_string(10));

        let id = self
            .module()
            .declare_data(&name, Linkage::Local, false, false)?;

        let mut desc = self.data_desc().clone();

        self.module().define_data(id, &mut desc);

        *self.data_desc() = desc;
        self.data_desc().clear();
        self.post_define()?;
        self.globals().insert(name, id);

        Ok(id)
    }
}
