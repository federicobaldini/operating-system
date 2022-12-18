#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
/// this function is the entry point, since the linker looks for a function
/// named `_start` by default
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
// this function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
