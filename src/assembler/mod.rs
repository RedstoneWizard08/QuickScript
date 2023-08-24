use crate::arch::Architecture;

pub mod aarch64;
pub mod arm;
pub mod x86;
pub mod x86_64;

pub fn assemble_code(code: String, arch: Architecture) -> String {
    match arch {
        Architecture::ARM => arm::assemble_code(code),
        Architecture::AARCH64 => aarch64::assemble_code(code),
        Architecture::X86 => x86::assemble_code(code),
        Architecture::X86_64 => x86_64::assemble_code(code),
    }
}
