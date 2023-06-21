; using 16-bit real mode
[bits 16]
; switch_to_pm
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
; init_pm
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
    ; step 7: Call a well-known label with useful code
    call BEGIN_PM 