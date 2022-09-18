#[no_mangle]
pub extern "C" fn product(a: i32, b: i32) -> i32 {
    println!("Rust -- a: {}, b: {}", a, b);
    a * b
}

#[no_mangle]
pub extern "C" fn ratio(a: i32, b: i32) -> i32 {
    println!("Rust -- a: {}, b: {}", a, b);
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = product(2, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_still_works() {
        let result = ratio(4, 2);
        assert_eq!(result, 2);
    }
}
