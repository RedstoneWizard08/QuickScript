use crate::{
    functions::{function::Function, print::Print},
    keyword::{AnyKeyword, KW_FN, KW_IF, KW_LET, KW_PRINT_WRAPPER, KW_RETURN},
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
        while self.tokens.get(self.position).is_some() {
            let mut token = self.read_token();

            if token.id == TOKENS.get("IDENT").unwrap().id {
                if let Some(value) = token.value {
                    if value == "fn" {
                        let name = self.read_token().value.unwrap();

                        self.read_token();

                        let mut args = Vec::new();
                        let mut ret: Option<Token> = None;
                        let mut block = Vec::new();
                        let mut token = self.read_token();
                        let mut remaining_closes = 1;

                        while token.value != Some(String::from(")")) {
                            args.push(token);
                            token = self.read_token();
                        }

                        let mut tmp = String::new();

                        token = self.read_token();

                        if let Some(ref v) = token.value {
                            tmp.push_str(v.as_str());
                        }

                        token = self.read_token();

                        if let Some(ref v) = token.value {
                            tmp.push_str(v.as_str());
                        }

                        if tmp == "->" {
                            ret = Some(self.read_token());

                            self.read_token();
                        }

                        while remaining_closes > 0 {
                            if token.value == Some(String::from("{")) {
                                remaining_closes += 1;
                            }

                            if token.value == Some(String::from("}")) {
                                remaining_closes -= 1;
                            }

                            if remaining_closes <= 0 {
                                break;
                            }

                            block.push(token);
                            token = self.read_token();
                        }

                        let block = Syntax::new(block).parse();

                        self.keywords
                            .push(AnyKeyword::Function(KW_FN.create((name, args, ret, block))));
                    } else if value == "if" {
                        self.read_token();

                        let mut condition = Vec::new();
                        let mut block = Vec::new();
                        let mut token = self.read_token();
                        let mut remaining_closes = 1;

                        while remaining_closes > 0 {
                            if token.value == Some(String::from("(")) {
                                remaining_closes += 1;
                            }

                            if token.value == Some(String::from(")")) {
                                remaining_closes -= 1;
                            }

                            if remaining_closes <= 0 {
                                break;
                            }

                            condition.push(token);
                            token = self.read_token();
                        }

                        self.read_token();
                        token = self.read_token();

                        remaining_closes = 1;

                        while remaining_closes > 0 {
                            if token.value == Some(String::from("{")) {
                                remaining_closes += 1;
                            }

                            if token.value == Some(String::from("}")) {
                                remaining_closes -= 1;
                            }

                            if remaining_closes <= 0 {
                                break;
                            }

                            block.push(token);
                            token = self.read_token();
                        }

                        let block = Syntax::new(block).parse();

                        self.keywords
                            .push(AnyKeyword::Block(KW_IF.create((condition, block))));
                    } else if value == "return" {
                        token = self.read_token();

                        self.keywords
                            .push(AnyKeyword::Return(KW_RETURN.create((token, String::new()))));
                    } else if value == "let" {
                        let name = self.read_token().value.unwrap();
                        let mut value = Vec::new();

                        self.read_token();

                        token = self.read_token();

                        while token.value != Some(String::from(";")) {
                            value.push(token);
                            token = self.read_token();
                        }

                        self.keywords
                            .push(AnyKeyword::Variable(KW_LET.create((name, value))));
                    } else if value == Print::name() {
                        self.read_token();

                        let mut tokens = Vec::new();

                        token = self.read_token();

                        while token.value != Some(String::from(")")) {
                            if token.value != Some(String::from(",")) {
                                tokens.push(token);
                            }

                            token = self.read_token();
                        }

                        let print_fn = Print::new(tokens);

                        self.keywords
                            .push(AnyKeyword::Print(KW_PRINT_WRAPPER.create(print_fn)));
                    }
                }
            }
        }

        self.keywords.clone()
    }
}
