use crate::{
    arch::{get_call_opcode, Architecture},
    compilable::Compilable,
    keyword::FunctionKeyword,
    token::Token,
    util::split_vec,
};

impl Compilable for FunctionKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let (name, args, _ret, block) = self.value.clone().unwrap();

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

        let mut _proc_arg_name = String::from("args");

        for arg in args.clone() {
            // Hard-coding this because I'm lazy.
            if arg.1[0].value == Some(String::from("str"))
                && arg.1[1].value == Some(String::from("["))
                && arg.1[2].value == Some(String::from("_"))
                && arg.1[3].value == Some(String::from("]"))
            {
                _proc_arg_name = arg.0.value.unwrap();
            }
        }

        let mut dbuf = String::new();
        let mut cbuf = String::new();

        cbuf.push_str(format!("{}:\n", name).as_str());

        for mut kw in block {
            let (d, c) = kw.to_asm(arch);

            dbuf.push_str(d.as_str());
            cbuf.push_str(c.as_str());
        }

        cbuf.push_str(format!("    {} exit\n", get_call_opcode(arch)).as_str());
        cbuf.push_str("    ret\n");

        if name == "main" {
            cbuf.push_str("_start:\n");
            cbuf.push_str(format!("    {} main\n", get_call_opcode(arch)).as_str());
            cbuf.push_str("    ret\n");
        }

        (dbuf, cbuf)
    }
}
