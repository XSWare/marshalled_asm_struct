section .text
extern rust_print
global asm_add
asm_add:
    add rdi, rsi
    mov rdx, rdi 
    mov rsi, len
    mov rdi, message
    call rust_print
    ret

section .data
message:
    db "result"
len: equ $ - message
