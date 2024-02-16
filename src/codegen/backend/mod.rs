use anyhow::Result;
use cranelift_codegen::ir::{types, GlobalValue, InstBuilder, Type, Value};
use cranelift_module::DataId;

use crate::ast::expr::ExprKind;

use self::{
    call::CallCompiler,
    literal::LiteralCompiler,
    ops::OperationCompiler,
    ret::ReturnCompiler,
    unify::BackendInternal,
    vars::{var::VariableCompiler, VariableExprCompiler},
};

pub mod call;
pub mod literal;
pub mod ops;
pub mod ret;
pub mod unify;
pub mod vars;

pub const RETURN_VAR: &str = "__func_return__";

pub trait Backend<'a>: BackendInternal<'a> {
    fn query_type(ty: String) -> Type;
    fn ptr(&mut self) -> Type;
    fn null(&mut self) -> Value;
    fn nullptr(&mut self) -> Value;
    fn compile(&mut self, expr: ExprKind) -> Result<Value>;
    fn get_global(&mut self, id: DataId) -> GlobalValue;
}

impl<'a, T: BackendInternal<'a>> Backend<'a> for T {
    fn query_type(ty: String) -> Type {
        match ty.as_str() {
            "i8" | "u8" => types::I8,
            "i16" | "u16" => types::I16,
            "i32" | "u32" => types::I32,
            "i64" | "u64" => types::I64,
            "i128" | "u128" => types::I128,
            "f32" => types::F32,
            "f64" => types::F64,
            "bool" => Type::int(1).unwrap(),
            "char" => Type::int(32).unwrap(),
            "str" => Type::int(32).unwrap(),

            _ => types::I32,
        }
    }

    fn ptr(&mut self) -> Type {
        self.module().target_config().pointer_type()
    }

    fn null(&mut self) -> Value {
        self.builder().borrow_mut().ins().null(types::I8)
    }

    fn nullptr(&mut self) -> Value {
        let ptr = self.ptr();
        
        self.builder().borrow_mut().ins().null(ptr)
    }

    fn get_global(&mut self, id: DataId) -> GlobalValue {
        let mut func = self.builder().borrow_mut().func.clone();
        let res = self.module().declare_data_in_func(id, &mut func);

        *self.builder().borrow_mut().func = func;

        res
    }

    fn compile(&mut self, expr: ExprKind) -> Result<Value> {
        match expr {
            ExprKind::None => Ok(self.null()),

            ExprKind::Literal(literal) => {
                let id = self.compile_literal(literal)?;
                let ptr = self.ptr();
                let data = self.get_global(id);

                Ok(self
                    .builder()
                    .borrow_mut().ins()
                    .symbol_value(ptr, data))
            }

            ExprKind::Call(call) => self.compile_call(call),
            ExprKind::Eof => Ok(self.null()),
            ExprKind::Identifer(ident) => self.compile_named_var(ident),
            ExprKind::Operation(op) => self.compile_op(op),
            ExprKind::Return(ret) => self.compile_return(ret),
            ExprKind::Variable(var) => self.compile_var(var),
        }
    }
}
