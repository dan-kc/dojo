// cargo test --lib concurrency::processing_pipeline::

//! Use three worker threads connected by bounded channels; each stage processes every input once and preserves order. Join all workers after channel closure. Arithmetic uses i32 and inputs are limited to -1000000..=1000000; division truncates toward zero.

/// Create a pipeline where data flows through multiple processing stages.
/// Stage 1: multiply by 2, Stage 2: add 10, Stage 3: divide by 3.
/// Each stage runs in its own thread connected by channels.
fn processing_pipeline(input: Vec<i32>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_pipeline() {
        let input = vec![6, 9, 12]; // (6*2+10)/3=22/3=7, (9*2+10)/3=28/3=9, (12*2+10)/3=34/3=11
        let result = processing_pipeline(input);
        assert_eq!(result, vec![7, 9, 11]);

        let input = vec![0];
        let result = processing_pipeline(input);
        assert_eq!(result, vec![3]); // (0*2+10)/3 = 10/3 = 3

        let empty: Vec<i32> = vec![];
        let result = processing_pipeline(empty);
        assert_eq!(result, Vec::<i32>::new());
    }
}
