use crate::lexer::parse;

pub const DATA: &str = include_str!("../test/main.qs");

#[test]
pub fn test_lexer() {
    parse("main.qs".into(), DATA.into());
}
