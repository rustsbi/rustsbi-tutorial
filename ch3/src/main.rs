#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings)]

linker::boot0!(rust_main; stack = 4096);

extern "C" fn rust_main(_hartid: usize, _dtb_ptr: usize) -> ! {
    // 清零 .bss
    unsafe { linker::zero_bss() };
    unsafe { &*(0x10_0000 as *const sifive_test_device::SifiveTestDevice) }.pass()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
