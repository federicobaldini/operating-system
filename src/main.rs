/*
    By default, all Rust crates link the standard library, which depends on the
    operating system for features such as threads, files, or networking. It also
    depends on the C standard library libc, which closely interacts with OS services.
    Since the plan is to write an operating system, I can’t use any OS-dependent
    libraries. So I have to disable the automatic inclusion of the standard library
    through the no_std attribute.
*/
#![no_std] // don't link the Rust standard library
/*
    In a typical Rust binary that links the standard library, execution starts in a C
    runtime library called crt0 (“C runtime zero”), which sets up the environment for
    a C application. This includes creating a stack and placing the arguments in the
    right registers. The C runtime then invokes the entry point of the Rust runtime,
    which is marked by the start language item. Rust only has a very minimal runtime,
    which takes care of some small things such as setting up stack overflow guards or
    printing a backtrace on panic. The runtime then finally calls the main function.

    My freestanding executable does not have access to the Rust runtime and crt0,
    so I need to define mine own entry point. Implementing the start language item
    wouldn’t help, since it would still require crt0. Instead, I need to overwrite
    the crt0 entry point directly.
*/
#![no_main] // disable all Rust-level entry points

/*
    The PanicInfo parameter contains the file and line where the panic happened
    and the optional panic message. The function should never return, so it is marked
    as a diverging function by returning the “never” type !. There is not much we
    can do in this function for now, so we just loop indefinitely.
*/
use core::panic::PanicInfo;

/*
    I am overwriting the operating system entry point with our own _start function.

    By using the #[no_mangle] attribute, I disable name mangling to ensure that
    the Rust compiler really outputs a function with the name _start. Without the attribute,
    the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol
    to give every function a unique name. The attribute is required because I need to tell
    the name of the entry point function to the linker.

    I also have to mark the function as extern "C" to tell the compiler that it should use
    the C calling convention for this function
    (instead of the unspecified Rust calling convention). The reason for naming the
    function _start is that this is the default entry point name for most systems.

    The ! return type means that the function is diverging, i.e. not allowed to ever return.
    This is required because the entry point is not called by any function, but invoked directly
    by the operating system or bootloader. So instead of returning, the entry point should e.g.
    invoke the exit system call of the operating system. In our case, shutting down the machine
    could be a reasonable action, since there’s nothing left to do if a freestanding
    binary returns. For now, we fulfill the requirement by looping endlessly.
*/
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/*
    The panic_handler attribute defines the function that the compiler should invoke
    when a panic occurs. The standard library provides its own panic handler function,
    but in a no_std environment we need to define it ourselves:
*/
#[panic_handler]
// this function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
