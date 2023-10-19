use std::path::PathBuf;

use rand::Rng;

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

pub fn random_string(len: i32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars = chars.chars();
    let n_chars = chars.clone().count();
    let mut val = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let ch = chars.clone().nth(rng.gen_range(0..n_chars)).unwrap();

        val.push(ch);
    }

    val
}

pub fn split_vec<T>(vec: Vec<T>, element: T) -> Vec<Vec<T>>
where
    T: PartialEq + Clone,
{
    let mut out = Vec::new();
    let mut cur = Vec::new();

    for item in vec {
        if item == element {
            out.push(cur.clone());
            cur = Vec::new();
        } else {
            cur.push(item);
        }
    }

    out.push(cur);

    out
}
