use crate::{
    functions::{function::Function, print::Print},
    keyword::{AnyKeyword, KW_EXIT, KW_FN, KW_PRINT_WRAPPER},
    token::{Token, TOKENS},
};

#[derive(Debug, Clone)]
pub struct Syntax {
    tokens: Vec<Token>,
    keywords: Vec<AnyKeyword>,
    position: usize,
}

impl Syntax {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            keywords: Vec::new(),
            position: 0,
        }
    }

    fn read_token(&mut self) -> Token {
        let token = self.tokens.get(self.position).unwrap().clone();

        self.position += 1;

        token
    }

    pub fn parse(&mut self) -> Vec<AnyKeyword> {
        while self.position < self.tokens.len() {
            let mut token = self.read_token();

            if token.id == TOKENS.get("IDENT").unwrap().id {
                if let Some(value) = token.value {
                    if value == "fn" {
                        let mut token = self.read_token();
                        let fn_name = token.clone();
                        let mut fn_args = Vec::new();

                        self.read_token();

                        while token.id != TOKENS.get("EXPR").unwrap().id
                            && token.value != Some(String::from(":"))
                            && token.value != Some(String::from(","))
                        {
                            token = self.read_token();

                            if token.id != TOKENS.get("EXPR").unwrap().id
                                && token.value != Some(String::from(":"))
                                && token.value != Some(String::from(","))
                            {
                                fn_args.push(token.clone());
                            }
                        }

                        self.read_token();
                        self.keywords.push(AnyKeyword::Token(KW_FN.create(fn_name)));
                    } else if value == "exit" {
                        token = self.read_token();

                        self.keywords.push(AnyKeyword::Token(KW_EXIT.create(token)));
                    } else if value == Print::name() {
                        self.read_token();

                        token = self.read_token();

                        self.read_token();

                        let print_fn = Print::new(token.value.unwrap());

                        self.keywords
                            .push(AnyKeyword::Print(KW_PRINT_WRAPPER.create(print_fn)));
                    }
                }
            }
        }

        self.keywords.clone()
    }
}
