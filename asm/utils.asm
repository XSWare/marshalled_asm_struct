section .text
extern rust_malloc
global count_character_asm

; rdi: str ptr
; rsi: len
; rdx: character we are searching for
count_character_asm:
    add rsi, rdi    ; rsi points to end of string
    xor r10, r10    ; set result register to 0
for_str_len:
    movsx rcx, byte [rsi]   ; load currently pointed at char
    mov r9, rsi
    sub r9, rdi             ; calculate diff 'current string pos - string start'
    cmp rcx, rdx            ; check if it is the char we are looking for
    jne not_symbol          ; skip generating result data if its not
    push r9         ; store offset on the stack
    inc r10         ; increase char count
not_symbol:
    dec rsi             ; point to the next char in the string
    test r9, r9         ; check if current position is at the string start
    jne for_str_len

allocate:
    mov rdi, r10
    push r10        ; the char count is the only register we need from the previous code
                    ; offsets are on the stack
    mov r8, rsp                     ; save stack ptr
    and rsp, 0xFFFFFFFFFFFFFFF0     ; align stack to 16 byte
    call rust_malloc                ; allocate array. rax has pointer to array start
    mov rsp, r8                     ; restore stack ptr
    pop r10         ; restore char count
    mov rdx, r10    ; set 'len' return value to char count/array length

store_results:
    mov rdi, rax    ; return value 'ptr' in rax contains array start ptr which we need a mutable copy of
continue:
    pop r9          ; restore an offset result
    mov [rdi], r9   ; and store it to the output array
    add rdi, 8      ; move array pointer to next element
    dec r10         ; decrement array len counter
    jne continue    ; and only return after it reaches 0
    ret
