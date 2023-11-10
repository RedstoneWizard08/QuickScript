use super::Type;
use cranelift::prelude::{types, Type as ClifType};
use phf::{phf_map, Map};

pub static PREDEFINED_TYPES: Map<&str, ClifType> = phf_map! {
    "VOID" => types::I8,
    "ANY" => types::I32,
    "str" => types::I64,
    "i8" => types::I8,
    "i16" => types::I16,
    "i32" => types::I32,
    "i64" => types::I64,
    "f32" => types::F32,
    "f64" => types::F64,
};

pub trait IntoClifType {
    fn into_clif_type(&self) -> Option<ClifType>;
}

impl IntoClifType for Type {
    fn into_clif_type(&self) -> Option<ClifType> {
        PREDEFINED_TYPES.get(&self.name).cloned()
    }
}
