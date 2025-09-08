pub mod file_io;
pub mod string_manipulation;
pub mod iterators;
pub mod lifetimes;
pub mod smart_pointers;

// New comprehensive practice modules
pub mod concurrency;
pub mod tokio;
pub mod collections;
pub mod ownership;
pub mod axum;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
