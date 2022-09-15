// use core_r;

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


#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct both_r {
    product: i32,
    sum: i32
}

impl both_r {
    pub fn new(a: i32, b: i32) -> both_r {
        both_r {
            product: core_r::product(a, b),
            sum: core_c::add_s(a, b)
        }
    }
}

#[no_mangle]
pub extern "C" fn both_r(a: i32, b: i32) -> both_r {
    both_r::new(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = both_r(7, 8);
        assert_eq!(result, both_r { product: 7*8, sum: 7+8 });
    }
}
