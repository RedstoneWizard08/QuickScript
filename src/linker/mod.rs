use crate::arch::Architecture;

pub mod aarch64;
pub mod arm;
pub mod x86;
pub mod x86_64;

pub fn link_binary(file: String, arch: Architecture) -> String {
    match arch {
        Architecture::ARM => arm::link_binary(file),
        Architecture::AARCH64 => aarch64::link_binary(file),
        Architecture::X86 => x86::link_binary(file),
        Architecture::X86_64 => x86_64::link_binary(file),
    }
}
