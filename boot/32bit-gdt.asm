; gdt_start
gdt_start:
    ; GDT starts with a null 8-byte
    dd 0x0 ; 4 byte
    dd 0x0 ; 4 byte

; GDT for code segment. base = 0x00000000, length = 0xfffff
gdt_code: 
    ; segment length, bits 0-15 (FROM 0-15)
    dw 0xffff
    ; segment base, bits 0-15 FROM(15-31)
    dw 0x0
    ; segment base, bits 16-23 FROM (16-23)
    db 0x0
    ; flags (8 bits) FROM (23-31)
    db 10011010b
    ; flags (4 bits) + segment length, bits 16-19 FROM(32 - 39)
    db 11001111b
    ; segment base, bits 24-31 FROM(40-47, total 48bits i.e. 6 bytes)
    ; (+ gdt_start )
    db 0x0

; GDT for data segment. base and length identical to code segment
gdt_data:
    dw 0xffff
    dw 0x0
    db 0x0
    db 10010010b
    db 11001111b
    db 0x0

gdt_end:
; GDT descriptor
gdt_descriptor:
    ; size (16 bit), always one less of its true size
    dw gdt_end - gdt_start - 1
    ; address (32 bit)
    dd gdt_start

; define some constants for later use
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start