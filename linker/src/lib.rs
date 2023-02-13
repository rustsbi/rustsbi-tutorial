//! 这个项目用于复用链接脚本文本和用于创建链接脚本文件的一小段代码。

#![deny(warnings, missing_docs)]

/// 创建链接脚本文件，返回文件路径。
pub fn linker_script() -> std::path::PathBuf {
    let ld = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    std::fs::write(&ld, LINKER).unwrap();
    ld
}

/// 链接脚本文本。
pub const LINKER: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    DRAM : ORIGIN = 0x80000000, LENGTH = 2M
}
SECTIONS {
    .text : {
        *(.text.entry)
        *(.text .text.*)
    } > DRAM
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > DRAM
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    } > DRAM
    .bss (NOLOAD) : {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(8);
        ebss = .;
    } > DRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
