section .text
global asm_add
asm_add:
        add rsi, rdi
        mov rax, rsi
        ret
