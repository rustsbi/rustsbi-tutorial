#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings)]

// #[macro_use]
// extern crate rcore_console;

use rcore_console::log::*;

linker::boot0!(rust_main; stack = 4096);

extern "C" fn rust_main(_hartid: usize, _dtb_ptr: usize) -> ! {
    // 清零 .bss
    unsafe { linker::zero_bss() };
    // 初始化 `console`
    rcore_console::init_console(&Console);
    rcore_console::set_log_level(option_env!("LOG"));
    rcore_console::test_log();
    unsafe { &*(0x10_0000 as *const sifive_test_device::SifiveTestDevice) }.pass()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{info}");
    loop {}
}

struct Console;

impl rcore_console::Console for Console {
    #[inline]
    fn put_char(&self, c: u8) {
        unsafe { (0x1000_0000 as *mut u8).write_volatile(c) };
    }
}
