fn main() {
    let ld = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    std::fs::write(&ld, linker::SCRIPT).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOG");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}
