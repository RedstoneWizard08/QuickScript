use std::process::exit;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    arch::Architecture,
    compilable::Compilable,
    functions::{function::Function, print::Print},
    token::{Token, TOKENS},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyKeyword {
    Token(Keyword<Token>),
    Print(Keyword<Print>),
}

impl Compilable for AnyKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        match self {
            AnyKeyword::Token(kw) => kw.to_asm(arch),
            AnyKeyword::Print(kw) => kw.to_asm(arch),
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
    pub static ref KW_EXIT: Keyword<Token> = Keyword::<Token> {
        id: 0,
        name: String::from("exit"),
        pretty_name: String::from("Exit"),
        documentation_key: String::from("#/doc/core/exit"),
        value: None,
    };

    /// The fn keyword. Defines a function. Wow.
    pub static ref KW_FN: Keyword<Token> = Keyword::<Token> {
        id: 1,
        name: String::from("fn"),
        pretty_name: String::from("Function"),
        documentation_key: String::from("#/doc/core/fn"),
        value: None,
    };

    pub static ref KW_PRINT_WRAPPER: Keyword<Print> = Keyword::<Print> {
        id: 2,
        name: String::from("print_wrap"),
        pretty_name: String::from("Print Wrapper"),
        documentation_key: String::from("#/doc/core/fn_wrappers/print"),
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

impl Compilable for Keyword<Token> {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let mut buf = String::new();

        if self.id == KW_EXIT.id {
            if let Some(value) = self.value.clone() {
                if arch == Architecture::ARM || arch == Architecture::AARCH64 {
                    buf.push_str("    mov w8, #93\n");
                    buf.push_str(format!("    mov x0, #{}\n", value.value.unwrap()).as_str());
                    buf.push_str("    svc #0\n");
                } else {
                    buf.push_str("    mov rax, 60\n");
                    buf.push_str(format!("    mov rdi, {}\n", value.value.unwrap()).as_str());
                    buf.push_str("    syscall\n");
                }
            }
        } else if self.id == KW_FN.id {
            if let Some(value) = self.value.clone() {
                if value.id != TOKENS.get("IDENT").unwrap().id {
                    eprintln!("A function name must be an identifier!");
                    exit(1);
                }

                let mut value = value.value.unwrap();

                if value == "main" {
                    value = String::from("_start");
                }

                buf.push_str(format!("{}:\n", value).as_str());
            }
        }

        (String::new(), buf)
    }
}

impl Compilable for Keyword<Print> {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        self.value.clone().unwrap().compile(arch)
    }
}
