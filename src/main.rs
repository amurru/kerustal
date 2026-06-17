#![no_std]
#![no_main]

use bootloader::entry_point;
use core::panic::PanicInfo;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
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

    // Initialize the layout state engine
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    loop {
        unsafe {
            // Read the status register
            let status = status_port.read();

            // Bit 0 of the status register is set to 1 if there is data waiting
            if status & 1 == 1 {
                // Read the actual scancode from the data register
                let scancode = data_port.read();

                // Pass the raw byte to the decoder engine
                if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                    // Translate the physical event into a logical keypress
                    if let Some(key) = keyboard.process_keyevent(key_event) {
                        match key {
                            // Normal character keys (handles lower/upper cases automatically)
                            DecodedKey::Unicode(character) => print!("{}", character),
                            // Function keys, arrow keys, etc.
                            DecodedKey::RawKey(key) => print!("{:?}", key),
                        }
                    }
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
