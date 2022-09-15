mod core {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub fn add_s(a: i32, b: i32) -> i32 {
        unsafe {
            add(a, b)
        }
    }

    pub fn both_c_s(a: i32, b: i32) -> both_c_t {
        unsafe {
            both_c(a, b)
        }
    }
}

fn main() {
    println!("Hello, world!");
    let a: i32 = 12;
    let b: i32 = 88;

    println!("Add: {}", core::add_s(a, b));
    println!("Product: {}", core_r::product(a, b));
    println!("Both_r: {:?}", wrapper_r::both_r(a, b));
    println!("Both_c: {:?}", core::both_c_s(a, b));
}
