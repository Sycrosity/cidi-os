#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(clippy::empty_loop)] //removes clippy errors from empty loops (which we will have at the start)

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

/// This function is called on panic, and in future should unwind and gracefully stop.
///
/// Currently, since stack unwinding isn't interpreted, in `Cargo.toml` we must set the panic strategy to abort - this means we don't have to (as we can't) implement the [`eh_personality` language item](https://github.com/rust-lang/rust/blob/edb368491551a77d77a48446d4ee88b35490c565/src/libpanic_unwind/gcc.rs#L11-L45).
///
/// Panic functions must return a [diverging function](https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions), shown here with the bang (!) operator.
///
/// The `_info` variable contains info about the line, row and file the error occured at - however, we cannot do anything with this data as of yet, as we have no printing or proper error handling as of yet.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

/// this should also return a divering function, much like the panic_handler [panic] also defined, as the entry point is never called and should instead invoke the [`exit` system call](https://en.wikipedia.org/wiki/Exit_(system_call)) of the OS (which we have not yet defined).
///
/// uses the [entry_point] macro, which provides a type checked way to define a non `pub extern "C" fn` or `#[no_mangle]` entry point function - it checks it at compile time so there is no runtime error (unlike with `pub extern "C" fn`).
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut value = 0x90;
        let mut yesno = true;
        for byte in framebuffer.buffer_mut() {
            yesno = !yesno;
            *byte = value;
            if yesno {
                value = value.wrapping_add(1);
            }
        }
    }
    loop {}
}
