// cargo test --lib iterators::dot_product::

//! Multiply corresponding entries and sum through the shorter slice. Unpaired trailing entries are ignored. Empty input yields 0.0; use O(1) extra storage.

pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_dot_product_empty() {
        let a = [];
        let b = [];
        let result = dot_product(&a, &b);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_dot_product_single_element() {
        let a = [3.0];
        let b = [7.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 21.0);
    }

    #[test]
    fn test_dot_product_different_lengths() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [5.0, 6.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 17.0); // 1*5 + 2*6 = 5 + 12 = 17 (zip stops at shorter length)
    }

    #[test]
    fn test_dot_product_negative_numbers() {
        let a = [1.0, -2.0, 3.0];
        let b = [4.0, 5.0, -6.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, -24.0); // 1*4 + (-2)*5 + 3*(-6) = 4 - 10 - 18 = -24
    }
}
