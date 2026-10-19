# file_io/binary_handler

Reference implementation. Read after attempting the exercise.

```rust
pub fn xor_encrypt_file(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    key: u8,
) -> std::io::Result<u64> {
    use std::io::{Read, Write};
    let mut input = std::fs::File::open(input_path)?;
    let mut output = std::io::BufWriter::new(std::fs::File::create(output_path)?);
    let mut buffer = [0u8; 8192];
    let mut count = 0;
    loop {
        let n = match input.read(&mut buffer) {
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            other => other?,
        };
        if n == 0 {
            break;
        }
        for byte in &mut buffer[..n] {
            *byte ^= key;
        }
        output.write_all(&buffer[..n])?;
        count += n as u64;
    }
    output.flush()?;
    Ok(count)
}
```
