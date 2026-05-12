#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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

