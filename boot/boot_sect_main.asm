[org 0x7c00]
    ; set the stack safely away from us
    mov bp, 0x8000 
    mov sp, bp
    ; buffer where the data is returned
    ; es:bx = 0x0000:0x9000 = 0x09000
    mov bx, 0x9000
    ; read 2 sectors
    mov dh, 2
    ; the bios sets 'dl' for our boot disk number
    call disk_load
    ; iterator used to read the buffer bx, start at bx address
    mov si, bx
    ; retrieve the first loaded word into dx
    mov dx, [si]
    ; print
    call print_hex
    call print_nl
    ; move to the next value
    add si, 512
    ; retrieve second loaded word
    mov dx, [si]
    ; print
    call print_hex
    call print_nl

    jmp $

%include "boot_sect_print.asm"
%include "boot_sect_print_hex.asm"
%include "boot_sect_disk.asm"

; Magic number
times 510 - ($-$$) db 0
dw 0xaa55

; boot sector = sector 1 of cyl 0 of head 0 of hdd 0
; from now on = sector 2 ...
; write 256 dx + 0xdada
times 256 dw 0xdada
; write 256 times dw + 0xface
times 256 dw 0xface