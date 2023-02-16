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
    // 清零 .bss
    unsafe { linker::zero_bss() };
    // 从设备树中解析出串口、测试设备的地址以及机器型号
    let machine = MachineInfo::from_dtb(dtb_ptr);
    unsafe {
        UART = machine.uart;
        TEST = machine.test;
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

struct MachineInfo {
    model: &'static str,
    uart: usize,
    test: usize,
}

impl MachineInfo {
    fn from_dtb(dtb_ptr: usize) -> Self {
        use core::mem::MaybeUninit;
        use dtb_walker::{Dtb, DtbObj, HeaderError as E, Property, Str, WalkOperation::*};

        let ans = MaybeUninit::<Self>::uninit();
        let mut ans = unsafe { ans.assume_init() };
        let dtb = unsafe {
            Dtb::from_raw_parts_filtered(dtb_ptr as _, |e| {
                matches!(e, E::Misaligned(4) | E::LastCompVersion(_))
            })
        }
        .unwrap();
        dtb.walk(|ctx, obj| match obj {
            DtbObj::SubNode { name } => {
                let current = ctx.name();
                if ctx.is_root() && name == Str::from("soc") {
                    StepInto
                } else if current == Str::from("soc")
                    && ["uart", "serial", "test", "clint"]
                        .iter()
                        .any(|pre| name.starts_with(pre))
                {
                    StepInto
                } else {
                    StepOver
                }
            }
            DtbObj::Property(Property::Model(model)) if ctx.is_root() => {
                let bytes = model.as_bytes();
                ans.model = unsafe {
                    core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                        bytes.as_ptr(),
                        bytes.len(),
                    ))
                };
                StepOver
            }
            DtbObj::Property(Property::Reg(mut reg)) => {
                let node = ctx.name();
                if node.starts_with("uart") || node.starts_with("serial") {
                    ans.uart = reg.next().unwrap().start;
                    StepOut
                } else if node.starts_with("test") {
                    ans.test = reg.next().unwrap().start;
                    StepOut
                } else {
                    StepOver
                }
            }
            DtbObj::Property(_) => StepOver,
        });
        ans
    }
}
