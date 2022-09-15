use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    if let Some(build_dirs) = env::var_os("LINK_PATHS") {
        for build_path in build_dirs.into_string().unwrap().split(',') {
            println!("cargo:rustc-link-search=native={}", build_path);
        }
    }
    println!("cargo:rustc-link-lib=core_c");

    println!("cargo:rerun-if-changed=ffi/core_c_wrapper.h");
    let mut bindgen_builder = bindgen::Builder::default();
    if let Some(include_dirs) = env::var_os("INCLUDE_PATHS") {
        for include_path in include_dirs.into_string().unwrap().split(',') {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", include_path));
        }
    }
    let bindings = bindgen_builder
        .header("ffi/core_c_wrapper.h")
        .clang_arg("-I/path")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed generating core_c bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}