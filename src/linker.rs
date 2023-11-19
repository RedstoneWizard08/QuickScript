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

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn get_dynamic_linker(
    prefix: String,
    target_arch: Option<String>,
    target_env: Option<String>,
) -> String {
    use std::env::consts::ARCH;

    use crate::target::ENV;

    let env = target_env.unwrap_or(ENV.to_string());

    if env == "android" {
        return "/system/lib/ld-android.so".to_string();
    }

    format!(
        "{}/lib/ld-linux-{}.so.1",
        prefix,
        target_arch.unwrap_or(ARCH.to_string())
    )
}

#[cfg(any(target_os = "linux", target_os = "android"))]
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
            target_arch.clone().unwrap_or(ARCH.to_string()),
            target_os.unwrap_or(OS.to_string()),
            target_env.clone().unwrap_or(ENV.to_string())
        ),
        "--dynamic-linker".to_string(),
        get_dynamic_linker(prefix, target_arch, target_env),
    ]
}

#[cfg(target_os = "windows")]
pub fn get_library_dir(_target_arch: String, _target_c: String) -> Vec<String> {
    todo!("get_library_dir is not supported on Windows yet!")
}

#[cfg(target_os = "windows")]
pub fn get_dynamic_linker(
    _prefix: String,
    _target_arch: Option<String>,
    _target_env: Option<String>,
) -> String {
    todo!("get_dynamic_linker is not supported on Windows yet!")
}
