use anyhow::Result;
use object::write::Object;

pub fn strip_binary(_bin: &mut Object) -> Result<()> {
    eprintln!("Binary stripping has not been implemented!");

    Ok(())
}
