use crate::parser::Lexer;

impl Lexer {
    pub fn interp_literal(&self, val: impl AsRef<str>) -> String {
        val.as_ref()
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
            .replace("\\0", "\0")
            .replace("\\\\", "\\")
            .replace("\\'", "'")
            .replace("\\\"", "\"")
    }
}
