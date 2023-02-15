#[macro_use]
extern crate clap;

use clap::Parser;
use once_cell::sync::Lazy;
use os_xtask_utils::{BinUtil, Cargo, CommandExt, Qemu};
use std::{
    fs,
    path::{Path, PathBuf},
};

const TARGET_ARCH: &str = "riscv64gc-unknown-none-elf";

static PROJECT: Lazy<&'static Path> =
    Lazy::new(|| Path::new(std::env!("CARGO_MANIFEST_DIR")).parent().unwrap());

static TARGET: Lazy<PathBuf> = Lazy::new(|| PROJECT.join("target").join(TARGET_ARCH));

#[derive(Parser)]
#[clap(name = "rCore-Tutorial")]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成文档。
    Book(BookArgs),
    /// 生成二进制文件。
    Make(BuildArgs),
    /// 生成反汇编文件。
    Asm(AsmArgs),
    /// 在 Qemu 中测试。
    Qemu(QemuArgs),
}

fn main() {
    use Commands::*;
    match Cli::parse().command {
        Book(args) => args.build(),
        Make(args) => {
            let _ = args.make();
        }
        Asm(args) => args.dump(),
        Qemu(args) => args.run(),
    }
}

#[derive(Args, Default)]
struct BookArgs {
    /// Chapter number
    #[clap(short, long)]
    ch: u8,
    /// Opens the book in a browser after the operation
    #[clap(long)]
    open: bool,
}

impl BookArgs {
    fn build(&self) {
        let package = format!("ch{}", self.ch);
        if self.open {
            Cargo::doc().package(&package).arg("--open").invoke();
        } else {
            Cargo::doc().package(&package).invoke();
            println!(
                "{}",
                TARGET
                    .join("doc")
                    .join(package)
                    .join("index.html")
                    .display()
            );
        }
    }
}

#[derive(Args, Default)]
struct BuildArgs {
    /// Chapter number
    #[clap(short, long)]
    ch: u8,
    /// Log level
    #[clap(long)]
    log: Option<String>,
    /// Builds in release mode
    #[clap(long)]
    release: bool,
}

impl BuildArgs {
    fn make(&self) -> PathBuf {
        let package = format!("ch{}", self.ch);
        // 生成
        Cargo::build()
            .package(&package)
            .optional(&self.log, |cargo, log| {
                cargo.env("LOG", log);
            })
            .conditional(self.release, |cargo| {
                cargo.release();
            })
            .target(TARGET_ARCH)
            .invoke();
        TARGET
            .join(if self.release { "release" } else { "debug" })
            .join(package)
    }
}

#[derive(Args)]
struct AsmArgs {
    #[clap(flatten)]
    build: BuildArgs,
    /// Output file.
    #[clap(short, long)]
    console: Option<String>,
}

impl AsmArgs {
    fn dump(self) {
        let elf = self.build.make();
        let out = Path::new(std::env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(self.console.unwrap_or(format!("ch{}.asm", self.build.ch)));
        println!("Asm file dumps to '{}'.", out.display());
        fs::write(out, BinUtil::objdump().arg(elf).arg("-d").output().stdout).unwrap();
    }
}

#[derive(Args)]
struct QemuArgs {
    #[clap(flatten)]
    build: BuildArgs,
    /// Path of executable qemu-system-x.
    #[clap(long)]
    qemu_dir: Option<String>,
    /// Number of hart (SMP for Symmetrical Multiple Processor).
    #[clap(long)]
    smp: Option<u8>,
    /// Port for gdb to connect. If set, qemu will block and wait gdb to connect.
    #[clap(long)]
    gdb: Option<u16>,
}

impl QemuArgs {
    fn run(self) {
        let elf = self.build.make();
        if let Some(p) = &self.qemu_dir {
            Qemu::search_at(p);
        }
        Qemu::system("riscv64")
            .args(&["-machine", "virt"])
            .arg("-nographic")
            .arg("-bios")
            .arg(objcopy(elf, true))
            // TODO
            // .arg("-kernel")
            // .arg(PROJECT.join("test-kernel.bin"))
            .args(&["-smp", &self.smp.unwrap_or(1).to_string()])
            .args(&["-m", "64M"])
            .args(&["-serial", "mon:stdio"])
            .optional(&self.gdb, |qemu, gdb| {
                qemu.args(&["-S", "-gdb", &format!("tcp::{gdb}")]);
            })
            .invoke();
    }
}

fn objcopy(elf: impl AsRef<Path>, binary: bool) -> PathBuf {
    let elf = elf.as_ref();
    let bin = elf.with_extension("bin");
    BinUtil::objcopy()
        .arg(elf)
        .arg("--strip-all")
        .conditional(binary, |binutil| {
            binutil.args(["-O", "binary"]);
        })
        .arg(&bin)
        .invoke();
    bin
}
