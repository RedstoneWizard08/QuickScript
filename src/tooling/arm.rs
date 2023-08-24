use crate::arch::{detect_arch, Architecture};

pub fn get_tools() -> (String, String) {
    let arch = detect_arch();

    if arch == Architecture::ARM {
        (String::from("ld"), String::from("as"))
    } else {
        (
            String::from("arm-linux-gnueabihf-ld"),
            String::from("arm-linux-gnueabihf-as"),
        )
    }
}
