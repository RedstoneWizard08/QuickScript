#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum Literal {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Char(char),

    #[default]
    None,
}
