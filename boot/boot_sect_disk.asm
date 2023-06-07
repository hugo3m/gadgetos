; @function disk_load
; @description load 'dh' sectors from drive 'dl' into ES:BX
; @parameter dh: number of sectors to read
; @return es:bx: value read from the disk
disk_load:
    pusha
    ; push dx into the stack
    ; dx = [dh; dl]
    ; pushing dx saves dh
    push dx

    mov ah, 0x02 ; ah <- int 0x13 function. 0x02 = 'read'
    ; number of sector to read from dh into al
    mov al, dh
    ; set sector to read   
    ; 0x01 is boot sector
    ; first sector possible to read is 0x02
    mov cl, 0x02
    ; set cylinder to read
    ; between [0x00 and 0x3FF]
    mov ch, 0x00
    ; set the drive number
    ; our caller sets it as a parameter and gets it from BIOS
    ; (0 = floppy, 1 = floppy2, 0x80 = hdd, 0x81 = hdd2)
    mov dh, 0x00 ; dh <- head number (0x0 .. 0xF)

    ; [es:bx] <- pointer to buffer where the data will be stored
    ; caller sets it up for us, and it is actually the standard location for int 13h
    ; BIOS interrupt
    int 0x13
    ; jump to disk_error if carry bit is 1 
    jc disk_error
    ; pop the value from the stack into dx (i.e. retrieve dh)
    pop dx
    ; BIOS also sets 'al' to the # of sectors read. 
    ; compare number of sectors read number of sector to read
    cmp al, dh
    ; if inequal go to secotrs_error
    jne sectors_error
    ; pop out stack into registers
    popa
    ; returns
    ret


disk_error:
    mov bx, DISK_ERROR
    call print
    call print_nl
    ; ah = error code, dl = disk drive that dropped the error
    mov dh, ah
    ; check out the code at http://stanislavs.org/helppc/int_13-1.html
    call print_hex 
    jmp disk_loop

sectors_error:
    mov bx, SECTORS_ERROR
    call print

disk_loop:
    jmp $

DISK_ERROR: db "Disk read error", 0
SECTORS_ERROR: db "Incorrect number of sectors read", 0