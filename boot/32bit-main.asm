; bootloader offset
[org 0x7c00]
    ; set the stack
    mov bp, 0x9000
    mov sp, bp

    ; print real mode message
    mov bx, MSG_REAL_MODE
    call print

    ; switch to 32-bit mode
    call switch_to_pm

%include "boot_sect_print.asm"
%include "32bit-gdt.asm"
%include "32bit-print.asm"
%include "32bit-switch.asm"

; 32 bit mode
[bits 32]
BEGIN_PM:
    ; ebx parameter
    mov ebx, MSG_PROT_MODE
    ; call function
    call print_string_pm
    ; infinite jump
    jmp $

MSG_REAL_MODE db "Started in 16-bit real mode", 0
MSG_PROT_MODE db "Loaded 32-bit protected mode", 0

; bootsector
times 510-($-$$) db 0
dw 0xaa55