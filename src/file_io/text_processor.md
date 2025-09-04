# Text File Processing Solution

## Solution

```rust
pub fn process_text_file(input_path: &std::path::Path, output_path: &std::path::Path) -> std::io::Result<usize> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    // Open input file with buffered reading for efficiency
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Create output file with buffered writing for efficiency
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut line_count = 0;

    // Process each line with automatic error propagation
    for line in reader.lines() {
        let line = line?; // Propagate any I/O errors
        line_count += 1;
        
        // Transform: add line number and convert to uppercase
        let transformed = format!("{}: {}\n", line_count, line.to_uppercase());
        writer.write_all(transformed.as_bytes())?;
    }

    // Ensure all data is flushed to disk
    writer.flush()?;
    Ok(line_count)
}
```

## Key Concepts Explained

- **Buffered I/O**: `BufReader` and `BufWriter` provide efficient buffering to minimize system calls
- **Error Propagation**: The `?` operator elegantly handles `Result` types and propagates errors
- **Iterator Pattern**: Using `reader.lines()` provides a clean, functional approach to line processing
- **Resource Management**: Rust's RAII ensures files are automatically closed when variables go out of scope
- **Explicit Flushing**: `writer.flush()` ensures all buffered data is written before the function returns