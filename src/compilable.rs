use serde::{Deserialize, Serialize};

/// An architecture.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Architecture {
    ARM,
    AARCH64,
    X86_64,
    I686,
}

/// Something that can translate/compile down to
/// assembly.
pub trait Compilable {
    /// Translate into assembly. Takes in the
    /// architecture to compile to.
    fn to_asm(&mut self, arch: Architecture) -> String;
}
