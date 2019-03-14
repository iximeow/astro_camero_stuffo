use std::collections::HashMap;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("linux") {
        panic!("Non-linux build targets are not tested and may not have a corresponding driver at the moment.");
    }

    let mut archmap: HashMap<&'static str, &'static str> = HashMap::new();
    archmap.insert("x86_64", "x64");
    archmap.insert("x86", "x86");
    archmap.insert("armv5", "armv5");
    archmap.insert("armv6", "armv6");
    archmap.insert("armv7", "armv7");
    archmap.insert("armv8", "armv8");
    for (arch, dir) in archmap.iter() {
        if target.contains(arch) {
            println!("cargo:rustc-link-search={}/lib/{}/", env::var("CARGO_MANIFEST_DIR").unwrap(), dir);
        }
    }
    println!("cargo:rustc-link-lib=ASICamera2");
    println!("cargo:rustc-link-lib=qhyccd");
}
