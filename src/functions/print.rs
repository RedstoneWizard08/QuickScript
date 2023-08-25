use serde::{Deserialize, Serialize};

use crate::{arch::Architecture, util::random_string};

use super::function::Function;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Print {
    args: Vec<String>,
}

impl Function for Print {
    fn new(value: String) -> Self {
        Self { args: vec![value] }
    }

    fn name() -> String {
        String::from("print")
    }

    fn compile(&self, arch: Architecture) -> (String, String) {
        let value = self.args.get(0).unwrap().clone();
        let mut buf = String::new();
        let mut data_buf = String::new();
        let msg_id = random_string(16);

        if arch == Architecture::ARM || arch == Architecture::AARCH64 {
            data_buf.push_str(format!("msg_{}:\n", msg_id).as_str());
            data_buf.push_str(format!("    .ascii \"{}\\n\"\n", value).as_str());
            data_buf.push_str(format!("len_{} = . - msg_{}\n", msg_id, msg_id).as_str());

            buf.push_str("    mov x0, #1\n");
            buf.push_str(format!("    ldr x1, =msg_{}\n", msg_id).as_str());
            buf.push_str(format!("    ldr x2, =len_{}\n", msg_id).as_str());
            buf.push_str("    mov w8, #64\n");
            buf.push_str("    svc #0\n");
        } else {
            data_buf.push_str(format!("msg_{} db \"{}\", 10\n", msg_id, value).as_str());
            data_buf.push_str(format!("len_{} equ $ - msg_{}\n", msg_id, msg_id).as_str());

            buf.push_str("mov rax, 1\n");
            buf.push_str("mov rdi, 1\n");
            buf.push_str(format!("mov rsi, msg_{}\n", msg_id).as_str());
            buf.push_str(format!("mov rdx, len_{}\n", msg_id).as_str());
            buf.push_str("syscall\n");
        }

        (data_buf, buf)
    }
}
