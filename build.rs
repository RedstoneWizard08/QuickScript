use std::{env, path::PathBuf, process::Command};

fn main() {
    if let Ok(git_hash) = env::var("GITHUB_SHA") {
        println!("cargo:rustc-env=COMMIT_HASH={}", &git_hash[0..7]);
    } else if PathBuf::from(".git").exists() {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap();

        let git_hash = &String::from_utf8(output.stdout).unwrap()[0..7];

        println!("cargo:rustc-env=COMMIT_HASH={}", git_hash);
    } else {
        println!("cargo:rustc-env=COMMIT_HASH=[crates.io]");
    }

    println!("cargo:rustc-env=PRODUCT_NAME=QuickScript");
}
