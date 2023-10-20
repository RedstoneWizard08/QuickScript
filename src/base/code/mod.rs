pub mod aarch64;
pub mod x86_64;

pub use aarch64::{AARCH64_EXIT, AARCH64_PRINT, AARCH64_SHIM};
pub use x86_64::{X86_64_EXIT, X86_64_PRINT, X86_64_SHIM};

pub const AARCH64_CODE_PREFIX: &str = include_str!("./aarch64.S");
pub const X86_64_CODE_PREFIX: &str = include_str!("./x86_64.asm");
