use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut lib_dir = manifest_dir.clone();
    lib_dir.push("dll");

    println!("cargo:rustc-link-search=all={}", lib_dir.display());
}
