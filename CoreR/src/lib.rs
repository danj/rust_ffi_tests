#[no_mangle]
pub extern "C" fn product(a: i32, b: i32) -> i32 {
    return a * b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = product(2, 3);
        assert_eq!(result, 6);
    }
}
