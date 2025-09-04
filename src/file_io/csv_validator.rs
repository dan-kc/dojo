// Exercise: CSV File Processing with Error Recovery
// Task: Read a CSV file, validate each row, process valid rows, and log errors.
// Write valid processed rows to output file and invalid rows to an error log.
//
// Hints:
// - Split by comma, but handle quoted fields properly
// - Validate that each row has the expected number of columns
// - Use BufReader/BufWriter for efficiency
// - Return counts of (valid_rows, invalid_rows)
//
// Example:
// Input: "name,age,city\nAlice,30,NYC\nBob,invalid,LA\nCarol,25,SF"
// Output: "NAME,AGE,CITY\nALICE,30,NYC\nCAROL,25,SF"
// Error log: "Line 3: Invalid age value: 'invalid'"
// Run tests with: cargo test --bin practice_csv_validator

pub fn process_csv_with_validation(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    error_log_path: &std::path::Path,
    expected_columns: usize,
) -> std::io::Result<(usize, usize)> {
    todo!("Implement CSV processing with validation and error logging")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, BufWriter, Write};
    use std::path::Path;

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_process_csv_with_validation() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("test_input.csv");
        let output_path = temp_dir.join("test_output.csv");
        let error_log_path = temp_dir.join("test_errors.log");

        // Create test CSV with some invalid rows
        let csv_content = "name,age,city\nAlice,30,NYC\nBob,invalid,LA\nCarol,25,SF\nDave,40\nEve,35,Boston,Extra";
        create_test_file(&input_path, csv_content.as_bytes()).unwrap();

        // Process CSV with validation
        let (valid, invalid) = process_csv_with_validation(
            &input_path,
            &output_path,
            &error_log_path,
            3, // Expected 3 columns
        )
        .unwrap();

        assert_eq!(valid, 3); // Alice, Carol, Eve (header not counted)
        assert_eq!(invalid, 2); // Bob (invalid age), Dave (missing column)

        // Verify output CSV
        let output = fs::read_to_string(&output_path).unwrap();
        assert!(output.contains("ALICE,30,NYC"));
        assert!(output.contains("CAROL,25,SF"));
        assert!(!output.contains("BOB")); // Invalid row excluded

        // Verify error log exists and contains error info
        let errors = fs::read_to_string(&error_log_path).unwrap();
        assert!(errors.contains("Line 3")); // Bob's line
        assert!(errors.contains("Line 5")); // Dave's line

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
        fs::remove_file(error_log_path).ok();
    }

    #[test]
    fn test_csv_with_quoted_fields() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("quoted.csv");
        let output_path = temp_dir.join("quoted_out.csv");
        let error_log_path = temp_dir.join("quoted_err.log");

        // CSV with quoted fields containing commas
        let csv_content = r#"name,description,price
"Smith, John","A person, with comma",100
"Invalid",,200
"Valid Product","Normal description",50"#;

        create_test_file(&input_path, csv_content.as_bytes()).unwrap();

        let (valid, invalid) =
            process_csv_with_validation(&input_path, &output_path, &error_log_path, 3).unwrap();

        // Should handle quoted fields properly
        assert_eq!(valid, 2); // "Smith, John" and "Valid Product"
        assert_eq!(invalid, 1); // "Invalid" has empty description field

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
        fs::remove_file(error_log_path).ok();
    }

    #[test]
    fn test_empty_csv() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("empty.csv");
        let output_path = temp_dir.join("empty_out.csv");
        let error_log_path = temp_dir.join("empty_err.log");

        // Create empty CSV
        create_test_file(&input_path, b"").unwrap();

        let (valid, invalid) =
            process_csv_with_validation(&input_path, &output_path, &error_log_path, 3).unwrap();

        assert_eq!(valid, 0);
        assert_eq!(invalid, 0);

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
        fs::remove_file(error_log_path).ok();
    }

    #[test]
    fn test_csv_header_only() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("header_only.csv");
        let output_path = temp_dir.join("header_out.csv");
        let error_log_path = temp_dir.join("header_err.log");

        // CSV with only header
        create_test_file(&input_path, b"name,age,city").unwrap();

        let (valid, invalid) =
            process_csv_with_validation(&input_path, &output_path, &error_log_path, 3).unwrap();

        assert_eq!(valid, 0);
        assert_eq!(invalid, 0);

        // Output should contain transformed header
        let output = fs::read_to_string(&output_path).unwrap();
        assert!(output.contains("NAME,AGE,CITY"));

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
        fs::remove_file(error_log_path).ok();
    }

    #[test]
    fn test_csv_all_invalid() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("all_invalid.csv");
        let output_path = temp_dir.join("all_invalid_out.csv");
        let error_log_path = temp_dir.join("all_invalid_err.log");

        // CSV with all invalid rows
        let csv_content = "name,age,city\nTooFew,1\nTooMany,2,3,4\n,,";
        create_test_file(&input_path, csv_content.as_bytes()).unwrap();

        let (valid, invalid) =
            process_csv_with_validation(&input_path, &output_path, &error_log_path, 3).unwrap();

        assert_eq!(valid, 0);
        assert_eq!(invalid, 3);

        // Error log should have all errors
        let errors = fs::read_to_string(&error_log_path).unwrap();
        assert!(errors.contains("Line 2"));
        assert!(errors.contains("Line 3"));
        assert!(errors.contains("Line 4"));

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
        fs::remove_file(error_log_path).ok();
    }
}
