pub mod code;
pub mod data;

pub use code::{
    AARCH64_CODE_PREFIX, AARCH64_EXIT, AARCH64_PRINT, AARCH64_SHIM, X86_64_CODE_PREFIX,
    X86_64_EXIT, X86_64_PRINT, X86_64_SHIM,
};

pub use data::{AARCH64_DATA_PREFIX, X86_64_DATA_PREFIX};

use crate::arch::Architecture;

pub fn build_code_prefix(arch: Architecture) -> String {
    match arch {
        Architecture::AARCH64 => [
            AARCH64_CODE_PREFIX,
            AARCH64_SHIM,
            AARCH64_EXIT,
            AARCH64_PRINT,
        ]
        .join("\n")
        .to_string(),

        Architecture::X86_64 => [X86_64_CODE_PREFIX, X86_64_SHIM, X86_64_EXIT, X86_64_PRINT]
            .join("\n")
            .to_string(),

        _ => todo!(),
    }
}

pub fn build_data_prefix(arch: Architecture) -> String {
    match arch {
        Architecture::AARCH64 => [AARCH64_DATA_PREFIX].join("\n").to_string(),
        Architecture::X86_64 => [X86_64_DATA_PREFIX].join("\n").to_string(),

        _ => todo!(),
    }
}
