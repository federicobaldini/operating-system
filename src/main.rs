// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  /*
     For printing “Hello World!”, we just need to know that the buffer is located
     at address 0xb8000 and that each character cell consists of
     an ASCII byte and a color byte.
    
    First, we cast the integer 0xb8000 into a raw pointer. Then we iterate over the bytes
    of the static HELLO byte string. We use the enumerate method to additionally get a
    running variable i. In the body of the for loop, we use the offset method to write the
    string byte and the corresponding color byte (0xb is a light cyan).

    Note that there’s an unsafe block around all memory writes. The reason is that the
    Rust compiler can’t prove that the raw pointers we create are valid. They could
    point anywhere and lead to data corruption. By putting them into an unsafe block,
    we’re basically telling the compiler that we are absolutely sure that the
    operations are valid. Note that an unsafe block does not turn off Rust’s safety checks.
    */
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  // this function is called on panic.
  loop {}
}
