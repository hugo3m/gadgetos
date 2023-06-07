# Boot

## Theory

#### Loop

When the computer boots, the BIOS doesn't know how to load the OS, so it delegates that task to the boot sector. Thus, the boot sector must be placed in a known, **standard location**. That location is the **first sector of the disk (cylinder 0, head 0, sector 0)** and it takes **512 bytes**.

To make sure that the "disk is bootable", the BIOS checks that **bytes 511 and 512** of the alleged boot sector **are bytes 0xAA55**.

This is the simplest boot sector ever:

```
e9 fd ff 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
[ 29 more lines with sixteen zero-bytes each ]
00 00 00 00 00 00 00 00 00 00 00 00 00 00 55 aa
```

It is basically **all zeros**, **ending with the 16-bit value 0xAA55** (beware of endianness, x86 is little-endian). **The first three bytes perform an infinite jump**.

You can either write the above 512 bytes with a binary editor, or just write a very simple assembler code:

```
; Infinite loop (e9 fd ff)
loop:
    jmp loop 

; Fill with 510 zeros minus the size of the previous code
times 510-($-$$) db 0
; Magic number
dw 0xaa55 
```
#### Print

Write each character of the "Hello" word into the register ```al``` (lower part of ```ax```), the bytes ```0x0e``` into ```ah``` (the higher part of ```ax```) and raise interrupt ```0x10``` which is a general interrupt for video services.

```0x0e``` on ```ah``` tells the video interrupt that the actual function we want to run is to 'write the contents of al in tty mode.

```ax``` is the primary accumulator; it is used in input/output and most arithmetic instructions. For example, in multiplication operation, one operand is stored in ```ax``` or ```ah``` or ```al``` register according to the size of the operand. ```ax``` is 16-bit register. ```al``` is 8-bit long from 0 to 7 and ```ah``` is 8-bit long from 8 to 15.

```
mov ah, 0x0e ; tty mode
mov al, 'H'
int 0x10
mov al, 'e'
int 0x10
mov al, 'l'
int 0x10
int 0x10 ; 'l' is still on al, remember?
mov al, 'o'
int 0x10

jmp $ ; jump to current address = infinite loop

; padding and magic number
times 510 - ($-$$) db 0
dw 0xaa55
```

#### Memory

The BIOS places the boot sector at ```0x7C00```. Therefore, when trying to access a variable, an offset of ```0x7C00``` must be taken into account. Here is the following code:

```
; tty mode
mov ah, 0x0e

; attempt 1
; Fails because it tries to print the memory address (i.e. pointer)
; not its actual contents
mov al, the_secret
int 0x10
; attempt 2
; Fails because it tries to print the memory address of 'the_secret' which is the correct approach.
; However, BIOS places our bootsector binary at address 0x7c00
mov al, [the_secret]
int 0x10
; attempt 3
; Success, add the BIOS starting offset 0x7c00 to the memory address of the X
; and then dereference the contents of that pointer.
; We need the help of a different register 'bx' because 'mov al, [ax]' is illegal.
; A register can't be used as source and destination for the same command.
mov bx, the_secret
add bx, 0x7c00
mov al, [bx]
int 0x10
; infinite loop
jmp $
the_secret:
    ; ASCII code 0x58 ('X') is stored just before the zero-padding.
    db "X"
; Fill with 510 zeros minus the size of the previous code
times 510-($-$$) db 0
; Magic number
dw 0xaa55
```

We can define a global offset for every memory location with ```[org 0x7C00]```. Here is the following code working:

```
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
```

#### Stack

It works like a basic stack. The stack uses ```bp``` register as base pointer for the stack and ```sp``` register as stack pointer. The stack goes from top to bottom, i.e. ```sp``` gets decremented.

#### Segmentation

Segmentation segments the memory into different zones, i.e. segments. It is a way to allow programs to address more than 64KB of memory. For doing so, specific registers such as:
* CS (code segment); 
* DS (data segment); 
* SS (current stack segment); 
* ES (extra segment).
are used to set the base address of a segment as well as a length. Each segment is 16 bits long i.e. can take value from 0x0000 to 0xFFFF. This size has been choosen because it is even. Initially, computers could only address 20 bits. Therefore, only 4 bits are left for the address of the segment. that's why to retrieve a real address you need to calculate `real_address = (segment << 4) + address`. 

```
# First address of first segment
segment = 0x0000
offset = 0x0000
real_address = (0x0000 << 4) + 0x0000
# Real address always on 20 bits
real_address = 0x00000 + 0x0000 = 0x00000
```

```
# Last address of first segment
segment = 0x0000
offset = 0xFFFF
real_address = (0x0000 << 4) + 0xFFFF
# Real address always on 20 bits (here 65535)
real_address = 0x00000 + 0xFFFF = 0x0FFFF
```

```
# First address of last segment
segment = 0xFFFF
offset = 0x0000
real_address = (0xFFFF << 4) + 0x0000
real_address = 0xFFFF0 + 0x0000 = 0xFFFF0
```

**Always be carefull with last segment, has a smaller size than other segments**

```
# Last address of last segment
segment = 0xFFFF
offset = 0x000F
real_address = (0xFFFF << 4) + 0x0000
real_address = 0xFFFF0 + 0x000F = 0xFFFFF
```

**Overflow example**

```
segment = 0xFFFF
offset = 0x0010
real_address = (0xFFFF << 4) + 0x0010
# Real address always on 20 bits
real_address = 0xFFFF0 + 0x0010 = 0x100000 = 0x00000
```

Segments can also overlap each other. First address of second segment is `(0x0001 << 4) + 0x0000 = 0x00010` but a middle address of first segment can be `(0x0000 << 4) + 0x0011 = 0x00011`. This problem is due because we have 16 bits for offset but only a 4 bits shift.

More info can be found here https://wiki.osdev.org/Segmentation.

#### Disk

Information of the physical organization of a disk: https://en.wikipedia.org/wiki/Cylinder-head-sector . Cylinder-head-sector is a 3D-coordinate system made out of:
* vertical coordinate *head*;
* horizontal coordinate *cylinder*;
* angular coordinate *sector*.

The following code is the routine to load value from disk. Basically, you specify into `dh` the number of sectors to load from the driver `dl`. More information can be found here https://stanislavs.org/helppc/int_13-2.html . The data will be stored in `ES:BX`.

```
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

    ; ah <- int 0x13 function. 0x02 = 'read'
    mov ah, 0x02 
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
```

The main function shows how to use the `disk_load` function:

```
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
```

## Compile

```
nasm -f bin boot.asm -o boot.bin
```

## Analyse binary

```
xxd boot.bin
```

## Run with QEMU

```
qemu-system-x86_64 boot.bin
```




