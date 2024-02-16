use std::{env, path::PathBuf, process::Command};

fn main() {
    if let Ok(git_hash) = env::var("GITHUB_SHA") {
        println!("cargo:rustc-env=COMMIT_HASH={}", &git_hash[0..7]);
    } else if PathBuf::from(format!("{}/.git", env!("CARGO_MANIFEST_DIR"))).exists() {
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
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/");
}
