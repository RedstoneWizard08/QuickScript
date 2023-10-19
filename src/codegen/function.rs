use crate::{
    arch::Architecture, compilable::Compilable, keyword::FunctionKeyword, token::Token,
    util::split_vec,
};

impl Compilable for FunctionKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let value = self.value.clone().unwrap();
        let name = value.0;
        let args = value.1;
        let block = value.2;

        let comma = Token {
            name: String::from("EXPR"),
            pretty_name: String::from("Expression"),
            id: 5,
            value: Some(String::from(".")),
        };

        let args = split_vec(args, comma);
        let args = args
            .iter()
            .cloned()
            .map(|v| (v[0].clone(), v[2..v.len()].to_vec()))
            .collect::<Vec<(Token, Vec<Token>)>>();

        let mut proc_arg_name = "args";

        for arg in args {
            // Hard-coding this because I'm lazy.
            if arg.1[0].value == Some(String::from("str"))
                && arg.1[1].value == Some(String::from("["))
                && arg.1[2].value == Some(String::from("_"))
                && arg.1[3].value == Some(String::from("]"))
            {
                proc_arg_name = arg.0.value.unwrap().as_str();
            }
        }

        (String::new(), String::new())
    }
}
