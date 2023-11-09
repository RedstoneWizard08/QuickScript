use crate::{
    arch::{get_call_opcode, Architecture},
    compilable::Compilable,
    keyword::FunctionKeyword,
};

impl Compilable for FunctionKeyword {
    fn to_asm(&mut self, arch: Architecture) -> (String, String) {
        let (name, _args, _ret, block) = self.value.clone().unwrap();

        let mut _proc_arg_name = String::from("args");
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
