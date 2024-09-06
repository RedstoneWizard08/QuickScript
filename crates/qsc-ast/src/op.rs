use crate::{expr::Expr, string_enum};

string_enum!(Operator = {
    Add: "+",
    Sub: "-",
    Mul: "*",
    Div: "/",
    Mod: "%",
    And: "&",
    Or: "|",
    Not: "~",
    Xor: "^",
    Pow: "^^",

    Assign: "=",
    AddAssign: "+=",
    SubAssign: "-=",
    MulAssign: "*=",
    DivAssign: "/=",
    ModAssign: "%=",
    AndAssign: "&=",
    OrAssign: "|=",
    NotAssign: "~=",
    XorAssign: "^=",
    PowAssign: "^^=",

    ShiftLeft: "<<",
    ShiftRight: ">>",

    BoolAnd: "&&",
    BoolOr: "||",
    BoolNot: "!",
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryOperation {
    pub value: Box<Expr>,
    pub negative: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOperation {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Unary(UnaryOperation),
    Binary(BinaryOperation),
}
