use crate::arch::Architecture;

pub mod aarch64;
pub mod arm;
pub mod x86;
pub mod x86_64;

pub fn get_tools(arch: Architecture) -> (String, String) {
    match arch {
        Architecture::ARM => arm::get_tools(),
        Architecture::AARCH64 => aarch64::get_tools(),
        Architecture::X86 => x86::get_tools(),
        Architecture::X86_64 => x86_64::get_tools(),
    }
}
