section .data

message db "Hello, x86_64!", 10
length equ $ - message

section .text

global _start

_start:
    mov rax, 1 ; Write is syscall #1
    mov rdi, 1 ; stdout is fd #1
    mov rsi, message ; Put string in rsi
    mov rdx, length ; Put string length in rdx
    syscall ; Syscall

    mov rax, 60 ; Exit is syscall #60
    mov rdi, 0 ; Put exit code 0 in rdi
    syscall ; Syscall
