# file_io/text_processor

Reference implementation. Read after attempting the exercise.

```rust
pub fn process_text_file(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
) -> std::io::Result<usize> {
    use std::io::{BufRead, Write};
    let input = std::io::BufReader::new(std::fs::File::open(input_path)?);
    let mut output = std::io::BufWriter::new(std::fs::File::create(output_path)?);
    let mut count = 0;
    for line in input.lines() {
        count += 1;
        writeln!(output, "{count}: {}", line?.to_uppercase())?;
    }
    output.flush()?;
    Ok(count)
}
```
