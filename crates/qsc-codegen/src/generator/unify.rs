use cranelift_module::{DataId, Module};
use miette::Result;
use parking_lot::RwLock;

use crate::context::CompilerContext;

pub trait BackendInternal<M: Module> {
    fn post_define(cctx: &RwLock<CompilerContext<M>>, id: DataId) -> Result<()>;
    fn is_jit() -> bool;
}
