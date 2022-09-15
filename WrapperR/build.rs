use std::env;

fn main() {
    let build_dir = env::var_os("EXTRA_LINK_PATH").unwrap().into_string().unwrap();
    println!("cargo:rustc-link-search=native={}", &build_dir);
}