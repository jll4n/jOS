#include "keyboard_map.h"

#define IDT_SIZE 256
#define KEYBOARD_DATA_PORT 0x60
#define KEYBOARD_STATUS_PORT 0x64

extern void load_idt(unsigned char *idt_ptr);
extern unsigned char read_port(unsigned short port);
extern void write_port(unsigned short port, unsigned char data);
extern void keyboard_handler();

struct IDT_entry {
    unsigned short int offset_lowerbits;
    unsigned short int selector;
    unsigned char zero;
    unsigned char type_attr;
    unsigned short int offset_higherbits;
};

struct IDT_entry IDT[IDT_SIZE];

volatile char *vidptr = (char*)0xb8000;
volatile unsigned int current_loc = 0;

unsigned char idt_ptr[6];

void idt_init(void)
{
	unsigned long keyboard_address;
	
    keyboard_address = (unsigned long)keyboard_handler;
    IDT[0x21].offset_lowerbits  = keyboard_address & 0xffff;
    IDT[0x21].selector          = 0x10;
    IDT[0x21].zero              = 0;
    IDT[0x21].type_attr         = 0x8e;
    IDT[0x21].offset_higherbits = (keyboard_address & 0xffff0000) >> 16;

    write_port(0x20, 0x11);
    write_port(0xA0, 0x11);
    write_port(0x21, 0x20);
    write_port(0xA1, 0x28);
    write_port(0x21, 0x00);
    write_port(0xA1, 0x00);
    write_port(0x21, 0x01);
    write_port(0xA1, 0x01);
    write_port(0x21, 0xff);
    write_port(0xA1, 0xff);

    unsigned short limit = sizeof(struct IDT_entry) * 256 - 1;
    unsigned int base = (unsigned int)IDT;

    idt_ptr[0] = limit & 0xff;
    idt_ptr[1] = (limit >> 8) & 0xff;
    idt_ptr[2] = base & 0xff;
    idt_ptr[3] = (base >> 8) & 0xff;
    idt_ptr[4] = (base >> 16) & 0xff;
    idt_ptr[5] = (base >> 24) & 0xff;

    load_idt(idt_ptr);
}

void kb_init(void)
{
    write_port(0x21, 0xFD);
}

void keyboard_handler_main(void)
{
    unsigned char status;
    unsigned char keycode; // ← unsigned !

    status = read_port(KEYBOARD_STATUS_PORT);
    if (status & 0x01) {
        keycode = read_port(KEYBOARD_DATA_PORT);

        write_port(0x20, 0x20); // EOI après lecture

        if (!(keycode & 0x80)){
        	vidptr[current_loc++] = keyboard_map[keycode];
        	vidptr[current_loc++] = 0x07;
        }
    }
    write_port(0x20, 0x20); // EOI
}

void print_char(char c) {
    if (c == '\n') {
        unsigned int line_width = 80 * 2;
        current_loc = (current_loc / line_width + 1) * line_width;
    } else {
        vidptr[current_loc++] = c;
        vidptr[current_loc++] = 0x07;
    }
}

void print_string(const char *s) {
    while (*s) {
        print_char(*s++);
    }
}

const char *str = "jOS booting...\n";

void kmain(void)
{

	print_string("jOS booting...\n");

    unsigned int i = 0;
    unsigned int j = 0;

    idt_init();
    kb_init();

    while (j < 80 * 25 * 2) {
        vidptr[j]   = ' ';
        vidptr[j+1] = 0x07;
        j += 2;
    }

    j = 0;
    while (str[j] != '\0') {
        vidptr[i]   = str[j];
        vidptr[i+1] = 0x07;
        ++j;
        i += 2;
    }

    current_loc = i;
}
