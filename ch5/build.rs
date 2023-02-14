fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LOG");
    println!(
        "cargo:rustc-link-arg=-T{}",
        linker::linker_script().display()
    );
}
