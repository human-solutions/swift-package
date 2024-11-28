uniffi::include_scaffolding!("mymath");

// #[uniffi::export]
pub fn rust_add(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rust_add(2, 2);
        assert_eq!(result, 4);
    }
}
