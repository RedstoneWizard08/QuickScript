use std::{env::consts::ARCH, process::exit};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// An architecture.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Architecture {
    ARM,
    AARCH64,
    X86_64,
    I686,
}

pub fn detect_arch() -> Architecture {
    match ARCH {
        "arm" => Architecture::ARM,
        "aarch64" => Architecture::AARCH64,
        "x86_64" => Architecture::X86_64,
        "x86" => Architecture::I686,

        _ => {
            eprintln!("Unsupported system architecture!");
            exit(1);
        }
    }
}
