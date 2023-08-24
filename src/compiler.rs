use lazy_static::lazy_static;

use crate::{arch::Architecture, compilable::Compilable, keyword::Keyword};

lazy_static! {
    pub static ref PREFIX_AARCH64: &'static str = include_str!("base/aarch64.S");
    pub static ref PREFIX_X86_64: &'static str = include_str!("base/x86_64.asm");
}

pub fn compile(keywords: Vec<Keyword>, arch: Architecture) -> String {
    let mut buf = String::new();

    let prefix = match arch {
        Architecture::ARM | Architecture::AARCH64 => PREFIX_AARCH64.to_string(),
        _ => PREFIX_X86_64.to_string(),
    };

    buf.push_str(format!("{}\n", prefix).as_str());

    for mut keyword in keywords {
        buf.push_str(format!("{}\n", keyword.to_asm(arch)).as_str());
    }

    buf
}
