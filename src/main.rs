#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  vga_buffer::print_something();
  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  // this function is called on panic.
  loop {}
}
