use which::which;

#[macro_export]
macro_rules! exists_return {
    ($cmd: expr) => {
        if command_exists($cmd) {
            return $cmd;
        }
    };
}

pub fn command_exists<T>(cmd: T) -> bool
where
    T: AsRef<str>,
{
    which(cmd.as_ref()).is_ok()
}

pub fn get_default_linker() -> &'static str {
    exists_return!("mold");
    exists_return!("lld");
    exists_return!("gold");
    exists_return!("ld");
    exists_return!("clang");
    exists_return!("gcc");

    "cc"
}

#[cfg(target_os = "linux")]
pub fn get_library_dir(
    prefix_dir: Option<String>,
    target_arch: Option<String>,
    target_os: Option<String>,
    target_env: Option<String>,
) -> Vec<String> {
    use std::env::consts::{ARCH, OS};

    use crate::target::ENV;

    let prefix = prefix_dir.unwrap_or(std::env::var("PREFIX").unwrap_or(String::from("/usr")));

    // TODO: mingw support

    vec![
        format!("-L{}/lib", prefix),
        format!(
            "-L{}/lib/{}-{}-{}",
            prefix,
            target_arch.clone().unwrap_or(ARCH.to_string()),
            target_os.clone().unwrap_or(OS.to_string()),
            target_env.clone().unwrap_or(ENV.to_string())
        ),
        format!(
            "-L{}/{}-{}-{}/lib",
            prefix,
            target_arch.unwrap_or(ARCH.to_string()),
            target_os.unwrap_or(OS.to_string()),
            target_env.unwrap_or(ENV.to_string())
        ),
    ]
}

#[cfg(target_os = "windows")]
pub fn get_library_dir(target_arch: String, target_c: String) -> Vec<String> {
    todo!("get_library_dir is not supported on Windows yet!")
}
