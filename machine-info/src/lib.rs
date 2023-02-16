//! 这个项目用于从设备树解析硬件信息。

#![no_std]
#![deny(warnings, missing_docs)]

use core::ops::Range;

/// 硬件信息。
pub struct MachineInfo {
    /// 机器型号。
    pub model: InlineString<64>,
    /// 串口地址。
    pub uart: Range<usize>,
    /// TestDevice 地址。
    pub test: Range<usize>,
}

/// 栈上字符串。
#[derive(Clone)]
pub struct InlineString<const N: usize>([u8; N], usize);

impl MachineInfo {
    /// 从设备树解析硬件信息。
    pub fn from_dtb(dtb_ptr: usize) -> Self {
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
                if ctx.is_root() {
                    if name == Str::from("soc") {
                        StepInto
                    } else {
                        StepOver
                    }
                } else if current == Str::from("soc") {
                    if name.starts_with("uart")
                        || name.starts_with("serial")
                        || name.starts_with("test")
                        || name.starts_with("clint")
                    {
                        StepInto
                    } else {
                        StepOver
                    }
                } else {
                    StepOver
                }
            }
            DtbObj::Property(Property::Model(model)) if ctx.is_root() => {
                let bytes = model.as_bytes();
                ans.model.0[..bytes.len()].copy_from_slice(bytes);
                ans.model.1 = bytes.len();
                StepOver
            }
            DtbObj::Property(Property::Reg(mut reg)) => {
                let node = ctx.name();
                if node.starts_with("uart") || node.starts_with("serial") {
                    ans.uart = reg.next().unwrap().clone();
                    StepOut
                } else if node.starts_with("test") {
                    ans.test = reg.next().unwrap().clone();
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
