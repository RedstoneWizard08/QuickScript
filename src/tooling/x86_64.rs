use crate::arch::{detect_arch, Architecture};

pub fn get_tools() -> (String, String) {
    let arch = detect_arch();

    if arch == Architecture::X86_64 {
        (String::from("ld"), String::from("nasm"))
    } else {
        (String::from("x86_64-linux-gnu-ld"), String::from("nasm"))
    }
}
