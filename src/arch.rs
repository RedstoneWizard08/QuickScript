use std::{env::consts::ARCH, process::exit};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// An architecture.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Architecture {
    ARM,
    AARCH64,
    X86_64,
    X86,
}

pub fn detect_arch() -> Architecture {
    match ARCH {
        "arm" => Architecture::ARM,
        "aarch64" => Architecture::AARCH64,
        "x86_64" => Architecture::X86_64,
        "x86" => Architecture::X86,

        _ => {
            eprintln!("Unsupported system architecture!");
            exit(1);
        }
    }
}

pub fn get_num_prefix(arch: Architecture) -> &'static str {
    if arch == Architecture::ARM || arch == Architecture::AARCH64 {
        return "#";
    }

    ""
}

pub fn get_input_register(arch: Architecture) -> &'static str {
    if arch == Architecture::ARM || arch == Architecture::AARCH64 {
        return "x0";
    }

    "rax"
}

pub fn get_call_opcode(arch: Architecture) -> &'static str {
    if arch == Architecture::ARM || arch == Architecture::AARCH64 {
        return "bl";
    }

    "call"
}
