// TODO: MacOS support

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use log::debug;
use target_lexicon::Triple;
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

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn get_default_linker() -> &'static str {
    exists_return!("mold");
    exists_return!("ld.lld");
    exists_return!("ld.gold");
    exists_return!("ld");
    exists_return!("clang");
    exists_return!("gcc");

    "cc"
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn get_dynamic_linker(
    prefix: String,
    _target_arch: Option<String>,
    target_env: Option<String>,
) -> String {
    use qsc_core::target::ENV;
    #[cfg(not(target_arch = "x86_64"))]
    use std::env::consts::ARCH;

    let env = target_env.unwrap_or(ENV.to_string());

    if env == "android" {
        return "/system/bin/linker64".to_string();
    }

    #[cfg(target_arch = "x86_64")]
    {
        format!("{}/lib64/ld-linux-x86-64.so.2", prefix)
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        format!(
            "{}/lib/ld-linux-{}.so.1",
            prefix,
            _target_arch.unwrap_or(ARCH.to_string())
        )
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn get_library_dir(
    prefix_dir: Option<String>,
    target_arch: Option<String>,
    target_os: Option<String>,
    target_env: Option<String>,
) -> Vec<String> {
    use qsc_core::target::ENV;
    use std::env::consts::{ARCH, OS};

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
        get_dynamic_linker(prefix, target_arch, target_env),
        #[cfg(target_os = "android")]
        {
            "--pie".to_string()
        },
        #[cfg(target_os = "android")]
        {
            "-L/system/lib64".to_string()
        },
    ]
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn run_linker(
    out_path: PathBuf,
    linker: Option<String>,
    tmp_file: PathBuf,
    triple: Triple,
) -> Result<()> {
    let linker = linker.unwrap_or(get_default_linker().to_string());

    let libs = get_library_dir(
        None,
        Some(triple.architecture.to_string()),
        Some(triple.operating_system.to_string()),
        Some(triple.environment.to_string()),
    );

    let cmd_str = format!(
        "{} -o {} {} -lc {}",
        linker,
        out_path.to_str().unwrap(),
        libs.join(" "),
        tmp_file.to_str().unwrap()
    );

    debug!("Running linker with command: {}", cmd_str);

    Command::new(linker)
        .arg("-o")
        .arg(out_path)
        .args(libs)
        .arg("-lc")
        .arg(tmp_file)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub const WINDOWS_LIBS: [&str; 2] = ["legacy_stdio_definitions.lib", "ucrt.lib"];

#[cfg(target_os = "windows")]
pub fn run_linker(
    out_path: PathBuf,
    _linker: Option<String>,
    tmp_file: PathBuf,
    _triple: Triple,
) -> Result<()> {
    use lnk::ShellLink;

    let path = PathBuf::from("C:/ProgramData/Microsoft/Windows/Start Menu/Programs/Visual Studio 2022/Visual Studio Tools/Developer PowerShell for VS 2022.lnk");
    let link = ShellLink::open(path).unwrap();

    let exe = link
        .link_info()
        .clone()
        .unwrap()
        .local_base_path()
        .clone()
        .unwrap();

    let args = link.arguments().clone().unwrap();
    let mut args = args.split(" ").collect::<Vec<&str>>();

    assert_eq!(args.remove(0), "-noe");
    assert_eq!(args.remove(0), "-c");

    let args = args.join(" ");

    let import_arg = args
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .replace("\"\"\"", "'");

    let lib_path_arg = import_arg.replace("}", "; echo $env:LIB; exit}");
    let path_arg = import_arg.replace("}", "; echo $env:PATH; exit}");

    let lib_path = Command::new(&exe)
        .arg("-noe")
        .arg("-c")
        .arg(lib_path_arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let lib_path = String::from_utf8(lib_path.stdout)?;
    let lib_path = lib_path.trim().split("\r\n").last().unwrap().trim();

    // Windows won't find the symbols if we try to use the x86 path
    let lib_path = lib_path.replace("\\x86", "\\x64");

    let path = Command::new(exe)
        .arg("-noe")
        .arg("-c")
        .arg(path_arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let path = String::from_utf8(path.stdout)?;
    let path = path.trim().split("\r\n").last().unwrap().trim();

    let tmp_file = tmp_file.to_str().unwrap();

    let cmd_str = format!(
        "link.exe /nologo /subsystem:windows /ignore:4210 {} /entry:_start /out:{} {}",
        &tmp_file,
        out_path.to_str().unwrap(),
        WINDOWS_LIBS.join(" ")
    );

    debug!("Running linker with command: {}", cmd_str);

    Command::new("link.exe")
        .arg("/nologo")
        .arg("/subsystem:windows")
        .arg("/ignore:4210") // I don't care if the .CRT section exists tbh
        .arg(tmp_file)
        .arg("/entry:_start")
        .arg(format!("/out:{}", out_path.to_str().unwrap()))
        .args(WINDOWS_LIBS)
        .env("LIB", lib_path)
        .env("PATH", path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}
