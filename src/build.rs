use std::fs;

use crate::{arch::Architecture, assembler::assemble_code, linker::link_binary};

pub fn build(name: String, code: String, arch: Architecture) {
    let file = assemble_code(code, arch);
    let file = link_binary(file, arch);

    fs::rename(file, name).unwrap();
}
