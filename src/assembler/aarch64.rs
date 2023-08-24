use std::{fs, process::Command};

use crate::{arch::Architecture, tooling::get_tools};

pub fn assemble_code(code: String) -> String {
    fs::write("_tmp.S", code).unwrap();

    let (_, assembler) = get_tools(Architecture::AARCH64);

    Command::new(assembler)
        .arg("-o")
        .arg("_tmp.o")
        .arg("_tmp.S")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    fs::remove_file("_tmp.S").unwrap();

    String::from("_tmp.o")
}
