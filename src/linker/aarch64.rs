use std::{fs, process::Command};

use crate::{arch::Architecture, tooling::get_tools, util::name_str_no_ext};

pub fn link_binary(file: String) -> String {
    let (linker, _) = get_tools(Architecture::AARCH64);
    let name = name_str_no_ext(file.clone());

    Command::new(linker)
        .arg("-s")
        .arg("-o")
        .arg(name.clone())
        .arg(file.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    fs::remove_file(file).unwrap();

    name
}
