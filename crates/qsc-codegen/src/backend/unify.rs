use anyhow::Result;
use cranelift_module::{DataId, Module};

use crate::context::CompilerContext;

pub trait BackendInternal<M: Module> {
    fn post_define<'a>(cctx: &mut CompilerContext<'a, M>, id: DataId) -> Result<()>;
}
