use crate::arch::Architecture;

/// Something that can translate/compile down to
/// assembly.
pub trait Compilable {
    /// Translate into assembly. Takes in the
    /// architecture to compile to.
    fn to_asm(&mut self, arch: Architecture) -> String;
}
