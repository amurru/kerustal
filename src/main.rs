#![no_std]
#![no_main]

use bootloader::entry_point;
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    // A pointer to the start of the VGA text buffer
    let vga_buffer = 0xb8000 as *mut u8;

    // The string we want to print represented as raw bytes
    let hello = b"Hello World!";

    // Loop through each byte in our string and write it to memory
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            // Write the character byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the color attribute byte (0x02 is green text on a black background)
            *vga_buffer.offset(i as isize * 2 + 1) = 0x02;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
