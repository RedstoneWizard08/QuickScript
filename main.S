.data

msg:
    .ascii "Hello, ARM64!"

len = . - msg

.text

.globl _start

print:
    mov x0, #1
    mov w8, #64
    svc #0
    ret

exit:
    mov w8, #93
    svc #0
    ret

_start:
    ldr x1, =msg
    ldr x2, =len
    bl print

    mov x0, #0
    bl exit
