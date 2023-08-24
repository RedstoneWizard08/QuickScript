section .data
section .text

global _start

_start:
    mov rax, 60
    mov rdi, 20
    syscall
