#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings)]

#[macro_use]
extern crate rcore_console;

static mut UART: usize = 0;
static mut TEST: usize = 0;

linker::boot0!(rust_main; stack = 4096 * 2);

extern "C" fn rust_main(hartid: usize, dtb_ptr: usize) -> ! {
    unsafe { linker::zero_bss() };
    let machine = machine_info::MachineInfo::from_dtb(dtb_ptr);
    unsafe {
        UART = machine.uart.start;
        TEST = machine.test.start;
    }
    rcore_console::init_console(&Console);
    rcore_console::set_log_level(option_env!("LOG"));
    println!(
        r"
___       __ __ _
 | . _   (_ |__)|
 | || |\/__)|__)|
-------/---------
machine: {machine}
memory: {mem:#x?}
boot hart: {hartid}
dtb region: {dtb:#x?}
",
        machine = machine.model,
        mem = machine.mem,
        dtb = machine.dtb,
    );
    shutdown()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rcore_console::log::error!("{info}");
    loop {}
}

struct Console;

impl rcore_console::Console for Console {
    fn put_char(&self, c: u8) {
        unsafe { (UART as *mut u8).write_volatile(c) };
    }
}

fn shutdown() -> ! {
    unsafe { &*(TEST as *const sifive_test_device::SifiveTestDevice) }.pass()
}
