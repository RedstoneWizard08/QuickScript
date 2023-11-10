use anyhow::Result;
use cranelift_module::{DataDescription, DataId, Linkage, Module};
use cranelift_object::ObjectModule;

pub fn create_data<T, C>(
    module: &mut ObjectModule,
    data_desc: &mut DataDescription,
    name: T,
    contents: C,
) -> Result<DataId>
where
    T: AsRef<str>,
    C: Into<Vec<u8>>,
{
    data_desc.define(contents.into().into_boxed_slice());

    let id = module.declare_data(name.as_ref(), Linkage::Export, true, false)?;

    module.define_data(id, &data_desc)?;
    data_desc.clear();

    Ok(id)
}
