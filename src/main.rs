#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(clippy::empty_loop)] //

use core::panic::PanicInfo;

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

///`#[no_mangle]` makes sure that rust's default compiler [name mangling](https://en.wikipedia.org/wiki/Name_mangling) is disabled and the function is really called _start() (required as the default entry point of most systems.)
///
/// this function has to be a `pub extern "C" fn` as it needs to use the [C calling convention](https://en.wikipedia.org/wiki/Calling_convention) for this function (instead of rust's unspecified compiler changed calling convention).
///
/// this should also return a divering function, much like the panic_handler [panic] also defined, as the entry point is never called and should instead invoke the (`exit` system call)[https://en.wikipedia.org/wiki/Exit_(system_call)] of the OS (which we have not yet defined).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
