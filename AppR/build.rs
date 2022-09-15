use std::env;

fn main() {
    let build_dir_var = env::var_os("EXTRA_LINK_PATH").unwrap();
    for build_path in build_dir_var.into_string().unwrap().split(',') {
        println!("cargo:rustc-link-search=native={}", build_path);
    }
    // println!("cargo:rustc-link-lib=wrapper_r");
    // println!("cargo:rustc-link-lib=wrapper_c");
    // println!("cargo:rustc-link-lib=core_c");
    // println!("cargo:rustc-link-lib=core_r");

}