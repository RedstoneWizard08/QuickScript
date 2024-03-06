use cranelift_module::{DataId, Module};
use miette::Result;

use crate::context::CompilerContext;

pub trait BackendInternal<'a, M: Module> {
    fn post_define(cctx: &mut CompilerContext<'a, M>, id: DataId) -> Result<()>;
}
