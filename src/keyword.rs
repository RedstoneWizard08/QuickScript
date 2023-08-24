use std::process::exit;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    compilable::{Architecture, Compilable},
    token::{Token, TOKENS},
};

/// A keyword. Contains the id (integer), the name,
/// the pretty name, the key for documentation
/// lookup, and the value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub id: i8,
    pub name: &'static str,
    pub pretty_name: &'static str,
    pub documentation_key: &'static str,
    pub value: Option<Token>,
}

lazy_static! {
    /// The exit keyword. This is a keyword and not a
    /// function because reasons.
    pub static ref KW_EXIT: Keyword = Keyword {
        id: 0,
        name: "exit",
        pretty_name: "Exit",
        documentation_key: "#/doc/core/exit",
        value: None,
    };

    /// The fn keyword. Defines a function. Wow.
    pub static ref KW_FN: Keyword = Keyword {
        id: 1,
        name: "fn",
        pretty_name: "Function",
        documentation_key: "#/doc/core/fn",
        value: None,
    };
}

impl Keyword {
    pub fn create(&self, value: Token) -> Keyword {
        Keyword {
            id: self.id,
            name: self.name.clone(),
            pretty_name: self.pretty_name.clone(),
            documentation_key: self.documentation_key.clone(),
            value: Some(value),
        }
    }
}

impl Compilable for Keyword {
    // Not using the arch yet, I want to focus
    // on making this work first. Will only
    // compile to aarch64 asm right now.
    fn to_asm(&mut self, _: Architecture) -> String {
        let mut buf = String::new();

        if self.id == KW_EXIT.id {
            if let Some(value) = self.value.clone() {
                buf.push_str("    mov w8, #93\n");
                buf.push_str(format!("    mov x0, #{}\n", value.value.unwrap()).as_str());
                buf.push_str("    svc #0\n");
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

        buf
    }
}
