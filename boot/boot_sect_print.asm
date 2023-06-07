; print function
print:
; push in every registry to stack
    pusha
; start
start:
    ; move value of bx address into al
    mov al, [bx]
    ; compare al and 0
    cmp al, 0 
    ; if equal jump to done
    je done
    ; tty code (because of pusha command)
    mov ah, 0x0e
    ; interrupts print value to screem
    int 0x10 
    ; increment pointer and do next loop
    add bx, 1
    ; loop to start
    jmp start
; done
done:
; pop out stack to registry
    popa
; return to call line
    ret
; print_nl function
print_nl:
    pusha
    ; tty code (because of pusha command)
    mov ah, 0x0e
    ; newline char
    mov al, 0x0a
    int 0x10
    ; carriage return
    mov al, 0x0d
    int 0x10
    popa
    ret