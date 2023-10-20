exit:
    ; Exit code is in rax.
    mov rdi, rax
    mov rax, 60
    syscall
    ret