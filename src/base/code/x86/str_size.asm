strsz:
    xor ecx, ecx
    not ecx
    xor al, al
    cld
    repnz scasb
    not ecx
    dec ecx
    mov edx, ecx
    ret
