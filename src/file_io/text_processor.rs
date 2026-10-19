// cargo test --lib file_io::text_processor::

//! Stream UTF-8 lines from a file into a distinct output file. Uppercase each line and prefix
//! its 1-based line number followed by ": ". Write a newline after every output line, including
//! the final one. Include blank lines, strip CRLF terminators, return the line count, and propagate
//! read/write/flush errors. Memory may scale with the longest line, not the entire input.

pub fn process_text_file(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> std::io::Result<usize> {
    todo!()
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
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
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
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
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
        let sandbox = tempfile::tempdir().unwrap();
        let missing = sandbox.path().join("missing");
        let target = sandbox.path().join("out");
        let non_existent = missing.as_path();
        let output = target.as_path();
        let result = process_text_file(non_existent, &output);
        assert!(result.is_err());
    }
    #[test]
    fn blank_crlf_and_unicode() {
        let d = tempfile::tempdir().unwrap();
        let a = d.path().join("a");
        let b = d.path().join("b");
        fs::write(&a, "straße\r\n\r\n雪").unwrap();
        assert_eq!(process_text_file(&a, &b).unwrap(), 3);
        assert_eq!(fs::read_to_string(b).unwrap(), "1: STRASSE\n2: \n3: 雪\n");
    }
}
