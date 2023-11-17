use anyhow::Result;
use cranelift_module::{DataDescription, DataId, Linkage, Module};

pub fn create_data<T, C, M>(
    module: &mut M,
    data_desc: &mut DataDescription,
    name: T,
    contents: C,
) -> Result<DataId>
where
    T: AsRef<str>,
    C: Into<Vec<u8>>,
    M: Module,
{
    data_desc.define(contents.into().into_boxed_slice());

    let id = module.declare_data(name.as_ref(), Linkage::Export, true, false)?;

    module.define_data(id, &data_desc)?;

    data_desc.clear();

    Ok(id)
}
