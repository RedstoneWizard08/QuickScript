use std::fs;

use lazy_static::lazy_static;
use tokio::process::Command;

use crate::{
    arch::{detect_arch, Architecture},
    compilable::Compilable,
    keyword::Keyword,
};

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

pub fn get_arm_tools() -> (String, String) {
    let arch = detect_arch();

    if arch == Architecture::ARM || arch == Architecture::AARCH64 {
        (String::from("ld"), String::from("as"))
    } else {
        (
            String::from("aarch64-linux-gnu-ld"),
            String::from("aarch64-linux-gnu-as"),
        )
    }
}

pub fn get_x86_tools() -> (String, String) {
    let arch = detect_arch();

    if arch == Architecture::X86_64 || arch == Architecture::I686 {
        (String::from("ld"), String::from("nasm"))
    } else {
        (String::from("x86_64-linux-gnu-ld"), String::from("nasm"))
    }
}

pub async fn assemble_and_link(name: String, code: String, arch: Architecture) {
    if arch == Architecture::ARM || arch == Architecture::AARCH64 {
        fs::write("_tmp.S", code).unwrap();

        let (linker, assembler) = get_arm_tools();

        Command::new(assembler)
            .arg("-o")
            .arg("_tmp.o")
            .arg("_tmp.S")
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();

        Command::new(linker)
            .arg("-s")
            .arg("-o")
            .arg("_tmp")
            .arg("_tmp.o")
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();

        fs::remove_file("_tmp.S").unwrap();
    } else {
        fs::write("_tmp.asm", code).unwrap();

        let (linker, assembler) = get_x86_tools();

        Command::new(assembler)
            .arg("-f")
            .arg("elf64")
            .arg("-o")
            .arg("_tmp.o")
            .arg("_tmp.asm")
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();

        Command::new(linker)
            .arg("-s")
            .arg("-o")
            .arg("_tmp")
            .arg("_tmp.o")
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap();

        fs::remove_file("_tmp.asm").unwrap();
    }

    fs::remove_file("_tmp.o").unwrap();
    fs::rename("_tmp", name).unwrap();
}
