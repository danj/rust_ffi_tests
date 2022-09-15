use std::env;
use std::path::Path;

fn main() {
    let build_dir_var = env::var_os("EXTRA_LINK_PATH").unwrap();
    let build_path = Path::new(&build_dir_var);
    println!("cargo:rustc-link-search=native={}", &build_path.display());
}