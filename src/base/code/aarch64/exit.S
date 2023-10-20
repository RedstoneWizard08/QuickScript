exit:
    ;// Exit code is in x0.

    mov w8, #93
    svc #0
    ret
