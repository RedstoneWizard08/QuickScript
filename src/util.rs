use std::path::PathBuf;

pub fn name_no_ext(path: PathBuf) -> String {
    let name = path.file_name().unwrap().to_str().unwrap();
    let mut name = name.split(".").collect::<Vec<&str>>();

    name.pop();

    name.join(".")
}

pub fn name_str_no_ext(name: String) -> String {
    let mut name = name.split(".").collect::<Vec<&str>>();

    name.pop();

    name.join(".")
}
