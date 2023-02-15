//! 书在 [`book`] 模块中。

#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
#![deny(warnings, missing_docs, rustdoc::broken_intra_doc_links)]

mod book;

static HELLO: &[u8] = b"Hello";
static WORLD: &[u8] = b", rustsbi!\n";

/// qemu-riscv64 的串口通常在这个地址。
const UART: usize = 0x1000_0000;

/// 应用程序的入口。
///
/// 裸机应用程序的最大特点就是必须自己准备自己的环境。
/// 由于还没有设置栈，入口必须是一个裸函数。
/// 编译器不会为裸函数插入栈操作指令，因此可以在任何情况下使用，比如这里。
/// 根据规范，裸函数中有且只能有一个 [`asm!`](core::arch::asm) 块。
/// 这个函数完成的具体工作见语句上的注释。
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    // 为高级语言准备的栈空间大小。
    const STACK_SIZE: usize = 4096;

    // 栈空间，放在一个专门的段上。
    #[link_section = ".bss.uninit"]
    static mut STACK: [u8; STACK_SIZE] = [0u8; STACK_SIZE];

    core::arch::asm!(
        // 这段汇编展示了打印的原始形态：从字符串存储的位置取出字节，传输给串口外设。
        "   li   a0, {uart}
            la   a1, {hello}
            ld   a1, (a1)
            addi a2, a1, 5
         0: lb   t0, (a1)
            sb   t0, (a0)
            addi a1, a1, 1
            blt  a1, a2, 0b
        ",
        // 进入高级语言环境最小的步骤：设置一个栈，然后跳转。
        "la sp, {stack} + {stack_size}",
        "j  {main}",
        // 第一段汇编的参数。
        uart  = const UART as usize,
        hello = sym HELLO,
        // 第二段汇编的参数。
        stack_size = const STACK_SIZE,
        stack      =   sym STACK,
        main       =   sym rust_main,
        // 根据规范，裸函数必须且只能带有 `noreturn` 选项。
        options(noreturn),
    )
}

/// 高级语言环境的入口。
extern "C" fn rust_main() -> ! {
    // 高级语言的编译器会使用栈，这里故意调用一个递归的打印函数来证明这一点。
    print_char_recu(WORLD);
    // 现在我们还不能控制计算机关机，
    print_char_recu(b"Press ctrl+a+x to terminate qemu.");
    loop {}
}

/// 一个递归的串口打印函数。
fn print_char_recu(bytes: &[u8]) {
    if let [head, tail @ ..] = bytes {
        unsafe { (UART as *mut u8).write_volatile(*head) };
        print_char_recu(tail);
    }
}

/// rust 语言级的异常处理，[`panic!`] 发生时调用的函数。
/// `no_std` 的应用程序必须设置，这里实际不会用到。
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    print_char_recu(b"panic!");
    unreachable!()
}
