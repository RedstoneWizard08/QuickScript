use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    arch::{get_call_opcode, get_input_register, get_num_prefix, Architecture},
    compilable::Compilable,
    functions::{function::Function, print::Print},
    token::Token,
};

pub type TokenKeyword = Keyword<Token>;
pub type PrintKeyword = Keyword<Print>;
pub type FunctionKeyword = Keyword<(String, Vec<(Token, Token)>, Option<Token>, Vec<AnyKeyword>)>;
pub type BlockKeyword = Keyword<(Vec<Token>, Vec<AnyKeyword>)>;
pub type VariableKeyword = Keyword<(String, String, Vec<Token>)>;
pub type ArrayKeyword = Keyword<(String, usize)>;
pub type ReturnKeyword = Keyword<(Token, String)>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyKeyword {
    Token(TokenKeyword),
    Print(PrintKeyword),
    Function(FunctionKeyword),
    Block(BlockKeyword),
    Variable(VariableKeyword),
    Return(ReturnKeyword),
}

impl Compilable for AnyKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        match self {
            AnyKeyword::Token(kw) => kw.to_asm(arch),
            AnyKeyword::Print(kw) => kw.to_asm(arch),
            AnyKeyword::Block(kw) => kw.to_asm(arch),
            AnyKeyword::Variable(kw) => kw.to_asm(arch),
            AnyKeyword::Function(kw) => kw.to_asm(arch),
            AnyKeyword::Return(kw) => kw.to_asm(arch),
        }
    }
}

/// A keyword. Contains the id (integer), the name,
/// the pretty name, the key for documentation
/// lookup, and the value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword<T> {
    pub id: i8,
    pub name: String,
    pub pretty_name: String,
    pub documentation_key: String,
    pub value: Option<T>,
}

lazy_static! {
    /// The exit keyword. This is a keyword and not a
    /// function because reasons.
    pub static ref KW_EXIT: TokenKeyword = TokenKeyword {
        id: 0,
        name: String::from("exit"),
        pretty_name: String::from("Exit"),
        documentation_key: String::from("#/doc/core/exit"),
        value: None,
    };

    /// The fn keyword. Defines a function. Wow.
    pub static ref KW_FN: FunctionKeyword = FunctionKeyword {
        id: 1,
        name: String::from("fn"),
        pretty_name: String::from("Function"),
        documentation_key: String::from("#/doc/core/fn"),
        value: None,
    };

    pub static ref KW_PRINT_WRAPPER: PrintKeyword = PrintKeyword {
        id: 2,
        name: String::from("print_wrap"),
        pretty_name: String::from("Print Wrapper"),
        documentation_key: String::from("#/doc/core/fn_wrappers/print"),
        value: None,
    };

    pub static ref KW_IF: BlockKeyword = BlockKeyword {
        id: 3,
        name: String::from("if"),
        pretty_name: String::from("Conditional"),
        documentation_key: String::from("#/doc/core/if"),
        value: None,
    };

    pub static ref KW_LET: VariableKeyword = VariableKeyword {
        id: 4,
        name: String::from("let"),
        pretty_name: String::from("Variable"),
        documentation_key: String::from("#/doc/core/let"),
        value: None,
    };

    pub static ref KW_RETURN: ReturnKeyword = ReturnKeyword {
        id: 5,
        name: String::from("return"),
        pretty_name: String::from("Return"),
        documentation_key: String::from("#/doc/core/return"),
        value: None,
    };
}

impl<T> Keyword<T> {
    pub fn create(&self, value: T) -> Keyword<T> {
        Keyword {
            id: self.id,
            name: self.name.clone(),
            pretty_name: self.pretty_name.clone(),
            documentation_key: self.documentation_key.clone(),
            value: Some(value),
        }
    }
}

impl Compilable for TokenKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let mut buf = String::new();

        if self.id == KW_EXIT.id {
            if let Some(value) = self.value.clone() {
                buf.push_str(
                    format!(
                        "mov {}, {}{}\n",
                        get_input_register(arch),
                        get_num_prefix(arch),
                        value.value.unwrap()
                    )
                    .as_str(),
                );

                buf.push_str(format!("{} exit\n", get_call_opcode(arch)).as_str());
            }
        }

        (String::new(), buf)
    }
}

impl Compilable for PrintKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        self.value.clone().unwrap().compile(arch)
    }
}

impl Compilable for BlockKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let mut dbuf = String::new();
        let mut cbuf = String::new();

        if let Some((_, ks)) = self.value.clone() {
            for mut k in ks {
                let (d, c) = k.to_asm(arch);

                dbuf.push_str(d.as_str());
                cbuf.push_str(c.as_str());
            }
        }

        (dbuf, cbuf)
    }
}

impl Compilable for VariableKeyword {
    fn to_asm(&mut self, _: Architecture) -> (String, String) {
        (String::new(), String::new())
    }
}

impl Compilable for ReturnKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let tkn = self.value.clone().unwrap().0;

        (
            String::new(),
            format!(
                "    mov {}, {}{}\n",
                get_input_register(arch),
                get_num_prefix(arch),
                tkn.value.unwrap()
            ),
        )
    }
}
