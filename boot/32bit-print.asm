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