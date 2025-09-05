// Dot Product
//
// Learning objectives:
// - Using zip() for parallel iteration
// - Efficient numerical computations with iterators
// - Understanding map and fold/sum patterns
//
// cargo test --bin dot_product

/// Implement a function that demonstrates efficient parallel-like processing
/// using iterator combinators. Given two vectors of the same length,
/// compute the dot product efficiently.
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    todo!("Use zip and map with fold/sum for optimal performance")
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

fn main() {
    println!("Run tests with: cargo test --bin dot_product");
}