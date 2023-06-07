; global offset memory location
[org 0x7c00]
; tty mode
mov ah, 0x0e
; move into al memory location of pointer the_secret
mov al, [the_secret]
; raise interrupt video service
int 0x10
; infinite loop
jmp $
the_secret:
    ; ASCII code 0x58 ('X') is stored just before the zero-padding.
    ; On this code that is at byte 0x2d (check it out using 'xxd file.bin')
    db "X"
; Fill with 510 zeros minus the size of the previous code
times 510-($-$$) db 0
; Magic number
dw 0xaa55