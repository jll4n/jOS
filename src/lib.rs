#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::arch::naked_asm;

mod vga_buffer;

pub const IDT_SIZE: usize = 256;
pub const KEYBOARD_DATA_PORT: u16 = 0x60;
pub const KEYBOARD_STATUS_PORT: u16 = 0x64;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C, packed)]
pub struct IDTEntry{
	pub offset_low: u16,
	pub selector: u16,
	pub zero: u8,
	pub type_attr: u8,
	pub offset_high: u16,
}

pub static mut IDT: [IDTEntry; IDT_SIZE] = [IDTEntry {
	offset_low: 0,
	selector: 0,
	zero: 0,
	type_attr: 0,
	offset_high: 0,
}; IDT_SIZE];

pub const vidptr: *mut u8 = 0xb8000 as *mut u8;
pub static mut current_loc: usize = 0;

pub unsafe extern "C" fn read_port(_port: u16) -> u8{
	asm!(
		"mov edx, [esp+4]",
		"in al, dx",
		"ret",
		options(noreturn)
	);
}

#[unsafe(naked)]
pub unsafe extern "C" fn write_port(_port: u16, value: u16){
	naked_asm!(
		"mov edx, [exp+4]",
		"mov al, [esp+8]",
		"out dx, al",
		"ret"
	)
}

#[unsafe(naked)]
pub unsafe extern "C" fn keyboard_handler(){
	naked_asm!(
		"pusha",
		"call keyboard_handler_main",
		"popa",
		"iretd"
	)
}

pub fn idt_init(){
	let keyboard_address: usize = keyboard_handler as usize;

	unsafe{
		IDT[0x21].offset_low = (keyboard_address & 0xFFFF) as u16;
		IDT[0x21].selector = 0x10;
		IDT[0x21].zero = 0;
		IDT[0x21].type_attr = 0x8E;
		IDT[0x21].offset_high = ((keyboard_address >> 16) & 0xFFFF) as u16;
	}

	unsafe{write_port(0x20, 0x11);}
	unsafe{write_port(0xA0, 0x11);}
	unsafe{write_port(0x21, 0x20);}
	unsafe{write_port(0xA1, 0x28);}
	unsafe{write_port(0x21, 0x00);}
	unsafe{write_port(0xA1, 0x00);}
	unsafe{write_port(0x21, 0x01);}
	unsafe{write_port(0xA1, 0x01);}
	unsafe{write_port(0x21, 0xFF);}
	unsafe{write_port(0xA1, 0xFF);}
}

pub fn kb_init(){
	unsafe{
		write_port(0x21, 0xFD);
	}
}

#[no_mangle]
pub extern "C" fn kmain() {
    print_string("jOS booting...\n");

    unsafe {
        idt_init();
        kb_init();
    }

    // Clear écran comme dans ton C
    unsafe {
        let vidptr = 0xb8000 as *mut u8;
        let mut j = 0;
        while j < 80 * 25 * 2 {
            core::ptr::write_volatile(vidptr.add(j), b' ');
            core::ptr::write_volatile(vidptr.add(j + 1), 0x07);
            j += 2;
        }
    }

    print_string("jOS booting...\n");
}

#[no_mangle]
pub extern "C" fn keyboard_handler_main() {
    unsafe {
        let status = read_port(KEYBOARD_STATUS_PORT);
        if status & 0x01 != 0 {
            let keycode = read_port(KEYBOARD_DATA_PORT);

            write_port(0x20, 0x20); // EOI

            if keycode & 0x80 == 0 {
                let c = keyboard_map(keycode);
                if c != 0 {
                    print_char(c as char);
                }
            }
        }
        write_port(0x20, 0x20); // EOI
    }
}

