// cargo test --lib file_io::csv_validator::

//! Validate a UTF-8 CSV file with exactly the header name,age,city. Use the csv package to handle
//! quoted commas and multiline fields. Each data record must have exactly three nonblank fields;
//! trim fields, and require age to parse as u8 and be at most 130. Write valid records with uppercase
//! name/city and canonical decimal age, preceded by NAME,AGE,CITY. For each invalid record, write
//! exactly "Record N: invalid data" followed by a newline, where the header is logical record 1.
//! Return (valid, invalid) data counts. An empty file or wrong header is InvalidData. CSV decoding
//! and filesystem errors propagate. Paths are distinct. Process records incrementally.

pub fn process_csv_with_validation(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    error_log_path: &std::path::Path,
) -> std::io::Result<(usize, usize)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quoted_records_and_schema() {
        let d = tempfile::tempdir().unwrap();
        let a = d.path().join("in");
        let b = d.path().join("out");
        let e = d.path().join("errors");
        std::fs::write(&a,"name,age,city\n\"Smith, Jane\",030,\"New\nYork\"\nBob,no,LA\nCarol,131,SF\nDave,40\nEve,35,Boston,Extra\n ,12,X\n雪,0,東京\n").unwrap();
        assert_eq!(process_csv_with_validation(&a, &b, &e).unwrap(), (2, 5));
        assert_eq!(std::fs::read_to_string(&e).unwrap(),"Record 3: invalid data\nRecord 4: invalid data\nRecord 5: invalid data\nRecord 6: invalid data\nRecord 7: invalid data\n");
        let mut r = csv::Reader::from_path(&b).unwrap();
        assert_eq!(
            r.headers().unwrap().iter().collect::<Vec<_>>(),
            ["NAME", "AGE", "CITY"]
        );
        let records: Vec<_> = r.records().map(Result::unwrap).collect();
        assert_eq!(
            records[0].iter().collect::<Vec<_>>(),
            ["SMITH, JANE", "30", "NEW\nYORK"]
        );
        assert_eq!(records[1].iter().collect::<Vec<_>>(), ["雪", "0", "東京"]);
    }
    #[test]
    fn empty_wrong_and_header_only() {
        let d = tempfile::tempdir().unwrap();
        let a = d.path().join("a");
        let b = d.path().join("b");
        let e = d.path().join("e");
        for text in ["", "name,city,age\n"] {
            std::fs::write(&a, text).unwrap();
            assert_eq!(
                process_csv_with_validation(&a, &b, &e).unwrap_err().kind(),
                std::io::ErrorKind::InvalidData
            );
        }
        std::fs::write(&a, "name,age,city\n").unwrap();
        assert_eq!(process_csv_with_validation(&a, &b, &e).unwrap(), (0, 0));
        assert_eq!(std::fs::read_to_string(b).unwrap(), "NAME,AGE,CITY\n");
    }
}
