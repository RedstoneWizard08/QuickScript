use crate::{
    arch::Architecture,
    base::{build_code_prefix, build_data_prefix},
    compilable::Compilable,
    keyword::AnyKeyword,
};

pub fn compile(keywords: Vec<AnyKeyword>, arch: Architecture) -> String {
    let mut buf = String::new();
    let mut data_buf = String::new();

    data_buf.push_str(format!("{}\n", build_data_prefix(arch)).as_str());
    buf.push_str(format!("{}\n", build_code_prefix(arch)).as_str());

    for mut keyword in keywords {
        let (data_buf_2, code_buf) = keyword.to_asm(arch);

        data_buf.push_str(format!("{}\n", data_buf_2).as_str());
        buf.push_str(format!("{}\n", code_buf).as_str());
    }

    format!("{}\n{}\n", data_buf, buf)
}
