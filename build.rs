use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = &String::from_utf8(output.stdout).unwrap()[0..7];

    println!("cargo:rustc-env=COMMIT_HASH={}", git_hash);
    println!("cargo:rustc-env=PRODUCT_NAME=QuickScript");
}
