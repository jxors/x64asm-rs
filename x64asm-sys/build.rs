use std::{env, path::PathBuf, process::{Command, Stdio}};

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=./cxx/interop.cc");
    println!("cargo:rerun-if-changed=./cxx/interop.h");
    println!("cargo:rerun-if-changed=./x64asm/src");
    println!("cargo:rerun-if-changed=./x64asm/include");
    println!("cargo:rerun-if-changed=./x64asm/Makefile");

    // Build x64asm
    let output = Command::new("make")
        .current_dir("./x64asm/")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert!(output.status.success(), "{} {}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout));

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib = dir.join("x64asm/lib/");
    assert!(lib.exists());
    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=x64asm");

    // Build interop
    cc::Build::new()
        .file("cxx/interop.cc")
        .include("./x64asm/")
        .warnings(false)
        .compile("interop");

    let bindings = bindgen::Builder::default()
        .header("./cxx/interop.h")
        .clang_arg("-I./x64asm/")
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_function("interop_.*")
        .allowlist_function("x64asm_.*")
        .allowlist_type("x64asm::.*")
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    println!("cargo:rustc-link-lib=stdc++");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}