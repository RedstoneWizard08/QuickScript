use crate::lexer::Lexer;

impl<'i> Lexer<'i> {
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
