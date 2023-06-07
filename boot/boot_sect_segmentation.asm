mov ah, 0x0e ; tty

; attempt to move value of the_secret pointer into al
mov al, [the_secret]
; print fail because offset of the boot sector
int 0x10
; "ds" is a segment register and you cannot directly move an immediate value into a segment register.
mov bx, 0x7c0
; moving value into data segment
mov ds, bx
; WARNING: from now on all memory references will be offset by 'ds' implicitly
mov al, [the_secret]
; print works
int 0x10
; move offset into bx
mov bx, 0x7c0
; use extra segment
mov es, bx
; all the following lines give the correct output
; i.e. all those mov instructions are equivalent
mov al, [es:the_secret]
int 0x10
mov al, [the_secret]
int 0x10
mov al, [ds:the_secret]
int 0x10


jmp $

the_secret:
    db "X"

times 510 - ($-$$) db 0
dw 0xaa55