section .text
global new
new:
    mov rax, say_hello
    mov QWORD [rdi], 3
    mov QWORD [rdi + 8], 42
    mov QWORD [rdi + 16], rax
    ret

align 8
say_hello:
    mov rsi, message_hello
    mov rdx, len_hello
    call print
    ret

print: 
    push rdi
    push rax
    mov rax, 1
    mov rdi, 1
    syscall
    pop rax
    pop rdi
    ret

section .data
message_new:
    db "struct created", 10
len_new: equ $ - message_new
message_hello:
    db "struct says hi", 10
len_hello: equ $ - message_hello
