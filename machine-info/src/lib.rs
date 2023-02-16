//! 这个项目用于从设备树解析硬件信息。

#![no_std]
#![deny(warnings, missing_docs)]

use core::{
    fmt::{Display, Formatter, Result},
    ops::Range,
};

/// 从设备树采集的板信息。
pub struct MachineInfo {
    /// 设备树地址范围。
    pub dtb: Range<usize>,
    /// 机器型号。
    pub model: InlineString<64>,
    /// CPU 核数。
    pub smp: usize,
    /// 内存地址范围。
    pub mem: Range<usize>,
    /// 串口地址范围。
    pub uart: Range<usize>,
    /// TestDevice 地址范围。
    pub test: Range<usize>,
    /// CLINT 地址范围。
    pub clint: Range<usize>,
}

/// 原地存储的有限长度字符串。
pub struct InlineString<const N: usize>(usize, [u8; N]);

impl<const N: usize> Display for InlineString<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", unsafe {
            core::str::from_utf8_unchecked(&self.1[..self.0])
        })
    }
}

impl MachineInfo {
    /// 从设备树解析机器信息。
    pub fn from_dtb(dtb_ptr: usize) -> Self {
        use dtb_walker::{Dtb, DtbObj, HeaderError as E, Property, Str, WalkOperation::*};

        const CPUS: &str = "cpus";
        const MEMORY: &str = "memory";
        const SOC: &str = "soc";
        const UART: &str = "uart";
        const SERIAL: &str = "serial";
        const TEST: &str = "test";
        const CLINT: &str = "clint";

        let mut ans = Self {
            dtb: dtb_ptr..dtb_ptr,
            model: InlineString(0, [0u8; 64]),
            smp: 0,
            mem: 0..0,
            uart: 0..0,
            test: 0..0,
            clint: 0..0,
        };
        let dtb = unsafe {
            Dtb::from_raw_parts_filtered(dtb_ptr as _, |e| {
                matches!(e, E::Misaligned(4) | E::LastCompVersion(_))
            })
        }
        .unwrap();
        ans.dtb.end += dtb.total_size();
        dtb.walk(|ctx, obj| match obj {
            DtbObj::SubNode { name } => {
                let current = ctx.name();
                if ctx.is_root() {
                    if name == Str::from(CPUS) || name == Str::from(SOC) || name.starts_with(MEMORY)
                    {
                        StepInto
                    } else {
                        StepOver
                    }
                } else if current == Str::from(SOC)
                    && ["uart", "serial", "test", "clint"]
                        .iter()
                        .any(|pre| name.starts_with(pre))
                {
                    StepInto
                } else {
                    if current == Str::from(CPUS) && name.starts_with("cpu@") {
                        ans.smp += 1;
                    }
                    StepOver
                }
            }
            DtbObj::Property(Property::Model(model)) if ctx.is_root() => {
                ans.model.0 = model.as_bytes().len();
                ans.model.1[..ans.model.0].copy_from_slice(model.as_bytes());
                StepOver
            }
            DtbObj::Property(Property::Reg(mut reg)) => {
                let node = ctx.name();
                if node.starts_with(UART) || node.starts_with(SERIAL) {
                    ans.uart = reg.next().unwrap();
                    StepOut
                } else if node.starts_with(TEST) {
                    ans.test = reg.next().unwrap();
                    StepOut
                } else if node.starts_with(CLINT) {
                    ans.clint = reg.next().unwrap();
                    StepOut
                } else if node.starts_with(MEMORY) {
                    ans.mem = reg.next().unwrap();
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
