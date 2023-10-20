print:
    mov x0, #1 ;// stdout is fd #1
    mov w8, #64 ;// Write is syscall #64
    svc #0 ;// Syscall
    ret
