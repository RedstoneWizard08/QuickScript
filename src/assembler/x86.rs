use std::{fs, process::Command};

use crate::{arch::Architecture, tooling::get_tools};

pub fn assemble_code(code: String) -> String {
    fs::write("_tmp.asm", code).unwrap();

    let (_, assembler) = get_tools(Architecture::X86);

    Command::new(assembler)
        .arg("-f")
        .arg("elf32")
        .arg("-o")
        .arg("_tmp.o")
        .arg("_tmp.asm")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    fs::remove_file("_tmp.asm").unwrap();

    String::from("_tmp.o")
}
