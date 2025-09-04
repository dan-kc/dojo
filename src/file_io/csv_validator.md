# CSV File Processing with Error Recovery Solution

## Solution

```rust
pub fn process_csv_with_validation(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    error_log_path: &std::path::Path,
    expected_columns: usize,
) -> std::io::Result<(usize, usize)> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    // Open files
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    
    let error_file = File::create(error_log_path)?;
    let mut error_writer = BufWriter::new(error_file);

    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut line_number = 0;
    let mut is_header = true;

    for line_result in reader.lines() {
        let line = line_result?;
        line_number += 1;

        // Parse CSV line with proper quote handling
        let fields = parse_csv_line(&line);

        if is_header {
            // Process header: transform to uppercase and write
            let header_fields: Vec<String> = fields.iter()
                .map(|field| field.to_uppercase())
                .collect();
            writeln!(writer, "{}", header_fields.join(","))?;
            is_header = false;
            continue;
        }

        // Validate field count
        if fields.len() != expected_columns {
            writeln!(error_writer, 
                "Line {}: Expected {} columns, found {}. Content: '{}'", 
                line_number, expected_columns, fields.len(), line
            )?;
            invalid_count += 1;
            continue;
        }

        // Validate field content (check for empty fields)
        let has_empty_fields = fields.iter().any(|field| field.trim().is_empty());
        if has_empty_fields {
            writeln!(error_writer, 
                "Line {}: Contains empty fields. Content: '{}'", 
                line_number, line
            )?;
            invalid_count += 1;
            continue;
        }

        // For this exercise, also validate that the second field (age) is numeric
        if expected_columns >= 2 {
            if let Err(_) = fields[1].parse::<u32>() {
                writeln!(error_writer, 
                    "Line {}: Invalid age value: '{}'. Content: '{}'", 
                    line_number, fields[1], line
                )?;
                invalid_count += 1;
                continue;
            }
        }

        // Transform valid row: convert to uppercase
        let transformed_fields: Vec<String> = fields.iter()
            .map(|field| field.to_uppercase())
            .collect();
        writeln!(writer, "{}", transformed_fields.join(","))?;
        valid_count += 1;
    }

    // Flush all writers
    writer.flush()?;
    error_writer.flush()?;

    Ok((valid_count, invalid_count))
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    // Add the last field
    fields.push(current_field.trim().to_string());
    fields
}
```

## Key Concepts Explained

- **Custom CSV Parsing**: Implementing quote-aware CSV parsing to handle fields containing commas
- **Multi-file Output**: Writing valid data to one file and errors to another simultaneously
- **Data Validation**: Multiple validation layers (field count, empty fields, data type validation)
- **Error Logging**: Comprehensive error messages with line numbers and context
- **State Tracking**: Using boolean flags to handle header rows differently
- **Iterator Processing**: Using `chars().peekable()` for character-by-character parsing with lookahead
- **String Manipulation**: Trimming whitespace and converting to uppercase for data normalization