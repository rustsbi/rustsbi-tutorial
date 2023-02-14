#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings)]

linker::boot0!(rust_main; stack = 4096);

extern "C" fn rust_main() -> ! {
    unreachable!()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
