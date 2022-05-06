#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // casting `0xb8000` into a raw pointer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // writes the string byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // writes the corresponding color byte; `0xb` is light cyan
        }
    }

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

