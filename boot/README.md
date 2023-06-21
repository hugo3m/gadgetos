# Boot

## Theory

#### Loop

When the computer boots, the BIOS doesn't know how to load the OS, so it delegates that task to the boot sector. Thus, the boot sector must be placed in a known, **standard location**. That location is the **first sector of the disk (cylinder 0, head 0, sector 0)** and it takes **512 bytes**.

To make sure that the "disk is bootable", the BIOS checks that **bytes 511 and 512** of the alleged boot sector **are bytes 0xAA55**.

This is the simplest boot sector ever:

```assembly
e9 fd ff 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
[ 29 more lines with sixteen zero-bytes each ]
00 00 00 00 00 00 00 00 00 00 00 00 00 00 55 aa
```

It is basically **all zeros**, **ending with the 16-bit value 0xAA55** (beware of endianness, x86 is little-endian). **The first three bytes perform an infinite jump**.

You can either write the above 512 bytes with a binary editor, or just write a very simple assembler code:

```assembly
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

```assembly
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

```assembly
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

```assembly
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

```c
# First address of first segment
segment = 0x0000
offset = 0x0000
real_address = (0x0000 << 4) + 0x0000
# Real address always on 20 bits
real_address = 0x00000 + 0x0000 = 0x00000
```

```c
# Last address of first segment
segment = 0x0000
offset = 0xFFFF
real_address = (0x0000 << 4) + 0xFFFF
# Real address always on 20 bits (here 65535)
real_address = 0x00000 + 0xFFFF = 0x0FFFF
```

```c
# First address of last segment
segment = 0xFFFF
offset = 0x0000
real_address = (0xFFFF << 4) + 0x0000
real_address = 0xFFFF0 + 0x0000 = 0xFFFF0
```

**Always be carefull with last segment, has a smaller size than other segments**

```c
# Last address of last segment
segment = 0xFFFF
offset = 0x000F
real_address = (0xFFFF << 4) + 0x0000
real_address = 0xFFFF0 + 0x000F = 0xFFFFF
```

**Overflow example**

```c
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

**Everyone knows that the IBM PC established 512-byte secotrs on floppies and hard disks as the standard.**

The following code is the routine to load value from disk. Basically, you specify into `dh` the number of sectors to load from the driver `dl`. More information can be found here https://stanislavs.org/helppc/int_13-2.html . The data will be stored in `ES:BX`.

```assembly
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

```assembly
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

#### 32-bit mode

Protected mode allows system software to use virtual memory, paging and multitasking. Protected mode may only be entered after the system software sets up one descriptor table and enables the Protection Enable (PE) bit in the control register 0 (CR0).

##### Feature

* Use 32 bit registers;
* Privilege level: In protected mode, there are four privilege levels or rings, numbered from 0 to 3, with ring 0 being the most privileged and 3 being the least;
* Segmentation: In protected mode, the segment is replaced by a 16-bit selector. The 13 upper bits (bit 3 to bit 15) contain the index of an entry inside a descriptor table. The next bit (bit 2) specifies whether the operation is used with the GDT or the LDT. The lowest two bits (bit 1 and bit 0) of the selector are combined to define the privilege of the request.

##### Drawbacks

We will lose BIOS interrupts and we'll need to code the GDT.

###### 32 bit print

```assembly
; using 32-bit protected mode
[bits 32]

; constant storing VGA memory address
VIDEO_MEMORY equ 0xb8000
; constant storing color for each character
WHITE_ON_BLACK equ 0x0f

; @function print_string_pm
print_string_pm:
    ; push all registers
    pusha
    ; edx beginning of video memory address
    mov edx, VIDEO_MEMORY

; IMPORTANT: In Assembly line of code are executing on after the other
; I.E when calling print_string_pm, the program continues with print_string_pm_loop

; @function print_string_pm_loop
print_string_pm_loop:
    ; [ebx] is the address of our char, move into al
    mov al, [ebx]
    ; move the character color into ah
    mov ah, WHITE_ON_BLACK
    ; cmp character with end of string
    cmp al, 0
    ; if equal jump to print_string_pm_done
    je print_string_pm_done
    ; else store character + character color into edx
    mov [edx], ax
    ; go to next char
    add ebx, 1
    ; go to next memory position (char + color)
    add edx, 2 ; next video memory position
    ; loop
    jmp print_string_pm_loop

; @function print_string_pm_done
print_string_pm_done:
    ; pop out all registers
    popa
    ; return
    ret
```

#### GDT

GDT vs Segmentation: rather than multiply the value of a segment register by 16 and then add to it the offset, a segment register becomes an index to a particular segment descriptor (SD). The following code defines the GDT in Assembly. GDT stands for Global Descriptor Table, here is its structure:
* Base address (32bits),which defines where the segment begins in physical memory;
* Segment Limit (20 bits), which defines the size of the segment;
* Various flags, which affect how the CPU interprets the segment, such as the privilige level of code that runs within it or whether it is read- or write-only.

The GDT descriptor describes the GDT and is a 6-byte structure containing:
* GDT size (16 bits);
* GDT address (32 bits).

**Warning**: The GDT structure fragments the base address and segment limit which is confusing. You can still refer to [os-dev](https://www.cs.bham.ac.uk//~exr/lectures/opsys/10_11/lectures/os-dev.pdf) document. For a small recap before reading the code:
* **DB**: define byte, 8 bits;
* **DW**: define word. Generally 2 bytes;
* **DD**: define double word.

Size of `0x00` is 1 byte 8 bits, size of `0x0` is 4 bits.

```assembly
; gdt_start
gdt_start:
    ; GDT starts with a null 8-byte
    dd 0x0 ; 4 byte
    dd 0x0 ; 4 byte

; GDT for code segment 
; base = 0x00000000 
; length = 0xfffff
gdt_code: 
    ; segment length [0:15]
    dw 0xffff
    ; segment base [0:15]
    dw 0x0
    ; segment base [16:23]
    db 0x0
    ; flags (8 bits)
    db 10011010b
    ; flags (4 bits) + segment length [16:19]
    db 11001111b
    ; segment base [bits 24-31]
    ; (+ gdt_start )
    db 0x0

; GDT for data segment
; base = 0x00000000 
; length = 0xfffff
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
```

#### Switching to protected mode *PM*

Here is the Assembly code to switch to *pm*.

```
; using 16-bit real mode
[bits 16]
; @function switch_to_pm
switch_to_pm:
    ; step 1: disable interrupts
    cli
    ; step 2: load the GDT descriptor
    lgdt [gdt_descriptor]
    ; step 3: set 32-bit mode bit in cr0
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    ; step 4: long jump by using a different segment
    ; CODE_SEG references to GDT
    jmp CODE_SEG:init_pm

; using 32-bit protected mode
[bits 32]
; @function init_pm
init_pm:
    ; step 5: update the segment registers
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    ; step 6: update the stack right at the top of the free space
    mov ebp, 0x90000 
    mov esp, ebp
    ; step 7: Call a well-known label
    ; IMPORTANT: it must be 32 bits code
    call BEGIN_PM 
```

The following main function uses **GDT** and **switch** to actually switch in 32 bit protected mode

```assembly
; bootloader offset
[org 0x7c00]
    ; set the stack
    mov bp, 0x9000
    mov sp, bp

    ; This will be written after the BIOS messages
    mov bx, MSG_REAL_MODE
    call print 

    ; switch to protected mode
    call switch_to_pm

; using 32-bit protected mode
[bits 32]
BEGIN_PM:
    ; string written at top-left corner 
    mov ebx, MSG_PROT_MODE
    call print_string_pm
    ; infinite jump
    jmp $

MSG_REAL_MODE db "Started in 16-bit real mode", 0
MSG_PROT_MODE db "Loaded 32-bit protected mode", 0

; bootsector
times 510-($-$$) db 0
dw 0xaa55
```

## Compile

```bash
nasm -f bin boot.asm -o boot.bin
```

## Analyse binary

```bash
xxd boot.bin
```

## Run with QEMU

```bash
qemu-system-x86_64 boot.bin
```

## Documentation

* [os-dev](https://www.cs.bham.ac.uk//~exr/lectures/opsys/10_11/lectures/os-dev.pdf)

