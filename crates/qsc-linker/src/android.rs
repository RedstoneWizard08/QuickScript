use qsc_core::target::ENV;
use std::env::consts::{ARCH, OS};

pub fn get_linker_args(
    prefix_dir: Option<String>,
    target_arch: Option<String>,
    target_os: Option<String>,
    target_env: Option<String>,
) -> Vec<String> {
    let prefix = prefix_dir.unwrap_or(std::env::var("PREFIX").unwrap_or(String::from("/usr")));

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
        super::get_dynamic_linker(prefix, target_arch, target_env),
        "--pie".to_string(),
        "-O2".to_string(),
        "-L/system/lib64".to_string(),
        "-lc".to_string(),
    ]
}
