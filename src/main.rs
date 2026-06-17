#![no_std]
#![no_main]

use bootloader::entry_point;
use core::panic::PanicInfo;

// Import out new vga_buffer module
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
    println!("Hi Cosmos from The Macro World!");
    println!("Are you {} coder? So what is {}?", 1337, 42);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
