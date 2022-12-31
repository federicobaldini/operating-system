#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;
use core::panic::PanicInfo;

#[no_mangle] // Don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  #[cfg(test)]
  test_main();

  loop {}
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

// This function is called on panic in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;
  unsafe {
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
  serial_print!("trivial assertion... ");
  assert_eq!(0, 1);
  serial_println!("[ok]");
}
