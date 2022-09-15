mod core_c {
    #[link(name = "core_c")]
    extern {
        fn add(a: i32, b: i32) -> i32;
    }

    pub fn add_s(a: i32, b: i32) -> i32 {
        unsafe {
            add(a, b)
        }
    }
}

mod wrapper_c {
    #[repr(C)]
    #[derive(Debug)]
    pub struct both_c_t {
        sum: i32,
        product: i32,
    }

    #[link(name = "wrapper_c")]
    extern {
        fn both_c(a: i32, b: i32) -> both_c_t;
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

    println!("Add: {}", core_c::add_s(a, b));
    println!("Product: {}", core_r::product(a, b));
    println!("Both_r: {:?}", wrapper_r::both_r(a, b));
    println!("Both_c: {:?}", wrapper_c::both_c_s(a, b));
}
