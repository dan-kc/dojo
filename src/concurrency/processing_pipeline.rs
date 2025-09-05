// Multi-Stage Processing Pipeline Practice
//
// Learning Objectives:
// - Create pipeline patterns with channels
// - Connect multiple processing stages
// - Understand data flow through thread boundaries
//
// cargo test --bin processing_pipeline

/// Create a pipeline where data flows through multiple processing stages.
/// Stage 1: multiply by 2, Stage 2: add 10, Stage 3: divide by 3.
/// Each stage runs in its own thread connected by channels.
fn processing_pipeline(input: Vec<i32>) -> Vec<i32> {
    todo!("Implement multi-stage processing pipeline")
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