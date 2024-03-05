use anyhow::Result;
use cranelift_module::{DataId, Module};

use crate::context::CompilerContext;

pub trait BackendInternal<'i, 'a, M: Module> {
    fn post_define(cctx: &mut CompilerContext<'i, 'a, M>, id: DataId) -> Result<()>;
}
