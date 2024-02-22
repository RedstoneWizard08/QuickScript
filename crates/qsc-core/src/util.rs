use std::path::PathBuf;

use rand::Rng;

pub fn split_vec<T>(vec: Vec<T>, item: T) -> Vec<Vec<T>>
where
    T: PartialEq,
{
    let mut res = Vec::new();
    let mut cur = Vec::new();

    for el in vec {
        if el == item {
            res.push(cur);
            cur = Vec::new();
            continue;
        }

        cur.push(el);
    }

    res
}

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

pub trait AsCharVec {
    fn as_char_vec(&self) -> Vec<char>;
}

impl<'a> AsCharVec for &'a str {
    fn as_char_vec(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl AsCharVec for String {
    fn as_char_vec(&self) -> Vec<char> {
        self.chars().collect()
    }
}
