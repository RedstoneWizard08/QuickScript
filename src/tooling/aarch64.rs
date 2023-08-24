use crate::arch::{detect_arch, Architecture};

pub fn get_tools() -> (String, String) {
    let arch = detect_arch();

    if arch == Architecture::AARCH64 {
        (String::from("ld"), String::from("as"))
    } else {
        (
            String::from("aarch64-linux-gnu-ld"),
            String::from("aarch64-linux-gnu-as"),
        )
    }
}
