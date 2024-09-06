use qsc_lexer::lexer::{parse, parse_tokens};

pub const DATA: &str = include_str!("../test/main.qs");

pub fn main() {
    pretty_env_logger::init();

    parse_tokens(
        "main.qs".into(),
        DATA.into(),
        format!("{}/output/out.tokens.ron", env!("CARGO_MANIFEST_DIR")).into(),
    );

    parse("main.qs".into(), DATA.into());
}
