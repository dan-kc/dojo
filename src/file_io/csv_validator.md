# file_io/csv_validator

Reference implementation. Read after attempting the exercise.

```rust
pub fn process_csv_with_validation(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    error_log_path: &std::path::Path,
) -> std::io::Result<(usize, usize)> {
    use std::io::Write;
    fn invalid(e: impl std::fmt::Display) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
    }
    let mut input = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(input_path)
        .map_err(invalid)?;
    if input.headers().map_err(invalid)?.iter().collect::<Vec<_>>() != ["name", "age", "city"] {
        return Err(invalid("expected name,age,city header"));
    }
    let mut output = csv::Writer::from_path(output_path).map_err(invalid)?;
    let mut errors = std::io::BufWriter::new(std::fs::File::create(error_log_path)?);
    output
        .write_record(["NAME", "AGE", "CITY"])
        .map_err(invalid)?;
    let (mut valid, mut rejected) = (0, 0);
    for (index, record) in input.records().enumerate() {
        let record = record.map_err(invalid)?;
        let fields: Vec<_> = record.iter().map(str::trim).collect();
        let age = fields
            .get(1)
            .and_then(|s| s.parse::<u8>().ok())
            .filter(|&n| n <= 130);
        if fields.len() != 3 || fields.iter().any(|s| s.is_empty()) || age.is_none() {
            rejected += 1;
            writeln!(errors, "Record {}: invalid data", index + 2)?;
        } else {
            output
                .write_record([
                    fields[0].to_uppercase(),
                    age.unwrap().to_string(),
                    fields[2].to_uppercase(),
                ])
                .map_err(invalid)?;
            valid += 1;
        }
    }
    output.flush()?;
    errors.flush()?;
    Ok((valid, rejected))
}
```
