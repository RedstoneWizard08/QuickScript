strlen_:
    // int strlen( const char * x0 /* s */ )

    // Set a reasonable maximum and branch to strnlen_s
    mov x1, #1000
    b strnlen_s

strnlen_s:
    // int strnlen_s( const char * x0 /* s */, int x1 /* max_len */ )

    // Register usage:
    //  x0 - Input pointer to string
    //  x1 - Max length we want to search
    // Intermediates:
    //  x2 - Running count of non-null bytes in string
    //  x3 - Place for testing if byte in string is null
    mov x2, #0

.L_strnlen_s_main_loop:
    ldrb w3, [x0], #+1
    cmp x3, #0
    b.eq .L_strnlen_s_exit
    add x2, x2, 1
    subs x1, x1, 1
    b.eq .L_strnlen_s_exit
    b .L_strnlen_s_main_loop

.L_strnlen_s_exit:
    mov x0, x2
    ret

/*
 * Performs a string length operation.
 * Inputs:
 * - x4 - The string to count (ldr x4, =string)
 * Outputs:
 * - x0 - The string length
 */
strlen:
    ldr x0, x4
    mov x1, #7
    bl strnlen_s
    ldr x0, x4
    bl strlen_
    ret
