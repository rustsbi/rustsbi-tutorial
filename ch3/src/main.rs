#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings)]

#[macro_use]
extern crate rcore_console;

static mut UART: usize = 0;
static mut TEST: usize = 0;

linker::boot0!(rust_main; stack = 4096 * 2);

extern "C" fn rust_main(_hartid: usize, dtb_ptr: usize) -> ! {
    use machine_info::MachineInfo;

    // 清零 .bss
    unsafe { linker::zero_bss() };
    // 从设备树中解析出串口、测试设备的地址以及机器型号
    let machine = MachineInfo::from_dtb(dtb_ptr);
    unsafe {
        UART = machine.uart.start;
        TEST = machine.test.start;
    }
    // 初始化 `console`
    rcore_console::init_console(&Console);
    rcore_console::set_log_level(option_env!("LOG"));
    // 在依赖 RustSBI 库之前我们还不能自称为 RustSBI，所以先随便起个名字，就叫 TinySBI 好了
    println!(
        r"
___       __ __ _
 | . _   (_ |__)|
 | || |\/__)|__)|
-------/---------
machine: {machine}
",
        machine = machine.model
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
