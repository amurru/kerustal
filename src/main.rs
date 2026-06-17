#![no_std]
#![no_main]

use bootloader::entry_point;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

// Import out new vga_buffer module
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    println!("KeRustal OS Initialized!");
    println!("Type something inside the QEMU window...");

    // Create ports mapped to the keyboard controller registers
    let mut status_port = Port::<u8>::new(0x64);
    let mut data_port = Port::<u8>::new(0x60);
    loop {
        unsafe {
            // Read the status register
            let status = status_port.read();

            // Bit 0 of the status register is set to 1 if there is data waiting
            if status & 1 == 1 {
                // Read the actual scancode from the data register
                let scancode = data_port.read();

                // For now, let's just print the raw hardware number!
                // NOTE: Scancodes below 0x80 mean a key was pressed, above 0x80 means released.
                if scancode < 0x80 {
                    print!("[Key:{:#X}]", scancode);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
