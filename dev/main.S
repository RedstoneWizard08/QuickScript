.data

msg:
    .ascii "Hello, ARM64!"

len = . - msg

.text

.global _start

_start:
    mov x0, #1 ;// stdout is fd #1
    ldr x1, =msg ;// Put string in x1
    ldr x2, =len ;// Put string length in x2
    mov w8, #64 ;// Write is syscall #64
    svc #0 ;// Syscall

    mov x0, #0 ;// Put exit code 0 in x0
    mov w8, #93 ;// Exit is syscall #93
    svc #0 ;// Syscall
