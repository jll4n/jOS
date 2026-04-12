bits 32
section .text
        align 4
        dd 0x1BADB002
        dd 0x00
        dd - (0x1BADB002 + 0x00)

global start
extern kmain

global read_port
global write_port
global load_idt
global keyboard_handler

extern keyboard_handler_main

start:
    cli
    mov esp, stack_space
    call kmain
    hlt

read_port:
	mov edx, [esp + 4]
	in al, dx	
	ret

write_port:
	mov   edx, [esp + 4]    
	mov   al, [esp + 4 + 4]  
	out   dx, al  
	ret

keyboard_handler:
    call keyboard_handler_main
    iretd

load_idt:
    mov edx, [esp + 4]
    lidt [edx]
    sti
    ret

section .bss
resb 8192
stack_space: