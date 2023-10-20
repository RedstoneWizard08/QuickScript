print:
    mov rax, 1 ; Write is syscall #1
    mov rdi, 1 ; stdout is fd #1
    syscall
    ret
