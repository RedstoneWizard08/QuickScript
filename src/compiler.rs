use lazy_static::lazy_static;

use crate::{arch::Architecture, compilable::Compilable, keyword::AnyKeyword};

lazy_static! {
    pub static ref PREFIX_AARCH64: &'static str = include_str!("base/code/aarch64.S");
    pub static ref PREFIX_X86_64: &'static str = include_str!("base/code/x86_64.asm");
    pub static ref PREFIX_DATA_AARCH64: &'static str = include_str!("base/data/aarch64.S");
    pub static ref PREFIX_DATA_X86_64: &'static str = include_str!("base/data/x86_64.asm");
}

pub fn compile(keywords: Vec<AnyKeyword>, arch: Architecture) -> String {
    let mut buf = String::new();
    let mut data_buf = String::new();

    let prefix = match arch {
        Architecture::ARM | Architecture::AARCH64 => PREFIX_AARCH64.to_string(),
        _ => PREFIX_X86_64.to_string(),
    };

    let data_prefix = match arch {
        Architecture::ARM | Architecture::AARCH64 => PREFIX_DATA_AARCH64.to_string(),
        _ => PREFIX_DATA_X86_64.to_string(),
    };

    data_buf.push_str(format!("{}\n", data_prefix).as_str());
    buf.push_str(format!("{}\n", prefix).as_str());

    for mut keyword in keywords {
        let (data_buf_2, code_buf) = keyword.to_asm(arch);

        data_buf.push_str(format!("{}\n", data_buf_2).as_str());
        buf.push_str(format!("{}\n", code_buf).as_str());
    }

    format!("{}\n{}\n", data_buf, buf)
}
