use crate::{
    compilable::{Architecture, Compilable},
    keyword::Keyword,
};

pub fn compile(keywords: Vec<Keyword>) -> String {
    let mut buf = String::new();

    buf.push_str(format!("{}\n", include_str!("base/aarch64.S")).as_str());

    for mut keyword in keywords {
        buf.push_str(format!("{}\n", keyword.to_asm(Architecture::AARCH64)).as_str());
    }

    buf
}
