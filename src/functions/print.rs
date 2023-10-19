use serde::{Deserialize, Serialize};

use crate::{arch::Architecture, token::Token, util::random_string};

use super::function::Function;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Print {
    args: Vec<Token>,
}

impl Function<Vec<Token>> for Print {
    fn new(args: Vec<Token>) -> Self {
        Self { args }
    }

    fn name() -> String {
        String::from("print")
    }

    fn compile(&self, arch: Architecture) -> (String, String) {
        let mut buf = String::new();
        let mut data_buf = String::new();
        let msg_id = random_string(16);

        if arch == Architecture::ARM || arch == Architecture::AARCH64 {
            for (idx, arg) in self.args.clone().iter().enumerate() {
                if arg.name == "STR_LIT" {
                    data_buf.push_str(format!("msg_{}_{}:\n", idx, msg_id).as_str());
                    data_buf.push_str(
                        format!("    .ascii \"{}\\n\"\n", arg.value.clone().unwrap()).as_str(),
                    );
                    data_buf.push_str(
                        format!("len_{}_{} = . - msg_{}_{}\n", idx, msg_id, idx, msg_id).as_str(),
                    );
                }
            }

            buf.push_str("    mov x0, #1\n");
            buf.push_str(format!("    ldr x1, =msg_{}\n", msg_id).as_str());
            buf.push_str(format!("    ldr x2, =len_{}\n", msg_id).as_str());
            buf.push_str("    mov w8, #64\n");
            buf.push_str("    svc #0\n");
        } else {
            for (idx, arg) in self.args.clone().iter().enumerate() {
                if arg.name == "STR_LIT" {
                    data_buf.push_str(
                        format!(
                            "msg_{}_{} db \"{}\", 10\n",
                            idx,
                            msg_id,
                            arg.value.clone().unwrap()
                        )
                        .as_str(),
                    );
                    data_buf.push_str(
                        format!("len_{}_{} equ $ - msg_{}_{}\n", idx, msg_id, idx, msg_id).as_str(),
                    );
                }
            }

            buf.push_str("mov rax, 1\n");
            buf.push_str("mov rdi, 1\n");
            buf.push_str(format!("mov rsi, msg_0_{}\n", msg_id).as_str());
            buf.push_str(format!("mov rdx, len_0_{}\n", msg_id).as_str());
            buf.push_str("syscall\n");
        }

        (data_buf, buf)
    }
}
