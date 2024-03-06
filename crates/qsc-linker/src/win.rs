use lnk::ShellLink;
use log::debug;
use miette::Result;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};
use target_lexicon::Triple;

pub const WINDOWS_LIBS: [&str; 2] = ["legacy_stdio_definitions.lib", "ucrt.lib"];
pub const POWERSHELL_LINK_PATH: &str = "C:/ProgramData/Microsoft/Windows/Start Menu/Programs/Visual Studio 2022/Visual Studio Tools/Developer PowerShell for VS 2022.lnk";

pub fn run_linker(
    out_path: PathBuf,
    _linker: Option<String>,
    tmp_file: PathBuf,
    _triple: Triple,
) -> Result<()> {
    let path = PathBuf::from(POWERSHELL_LINK_PATH);
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
        .arg("/ignore:4210") // I don't care if the .CRT section exists or not tbh
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
