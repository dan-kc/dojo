// Exercise: Text File Processing
// Task: Read a text file line by line, transform each line (uppercase and add line numbers),
// and write the result to a new file. Use BufReader for efficient reading.
//
// Hints:
// - Use BufReader::new() to wrap a File for buffered reading
// - Use lines() iterator for line-by-line processing
// - Remember to handle potential I/O errors with ?
//
// Example:
// Input file content:
//   hello world
//   rust is great
// Output file content:
//   1: HELLO WORLD
//   2: RUST IS GREAT
// Run tests with: cargo test --bin practice_text_processor

pub fn process_text_file(input_path: &std::path::Path, output_path: &std::path::Path) -> std::io::Result<usize> {
    todo!("Implement text file processing with line numbers and uppercase transformation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Write};
    use std::path::Path;

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_process_text_file() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("test_input.txt");
        let output_path = temp_dir.join("test_output.txt");

        // Create test input
        create_test_file(&input_path, b"hello world\nrust is great\nfile i/o").unwrap();

        // Process the file
        let lines_processed = process_text_file(&input_path, &output_path).unwrap();
        assert_eq!(lines_processed, 3);

        // Verify output
        let output = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output, "1: HELLO WORLD\n2: RUST IS GREAT\n3: FILE I/O\n");

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_empty_file() {
        let temp_dir = std::env::temp_dir();
        let empty_file = temp_dir.join("empty.txt");
        create_test_file(&empty_file, b"").unwrap();
        let output = temp_dir.join("empty_output.txt");

        let result = process_text_file(&empty_file, &output).unwrap();
        assert_eq!(result, 0);

        // Cleanup
        fs::remove_file(empty_file).ok();
        fs::remove_file(output).ok();
    }

    #[test]
    fn test_non_existent_file() {
        let non_existent = Path::new("/tmp/does_not_exist_12345.txt");
        let output = Path::new("/tmp/output.txt");
        let result = process_text_file(non_existent, &output);
        assert!(result.is_err());
    }
}
