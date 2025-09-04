// Exercise: Binary File Operations
// Task: Read a binary file in chunks, XOR each byte with a key value,
// and write the encrypted result to a new file using BufWriter.
//
// Hints:
// - Use a fixed buffer size (e.g., 8192 bytes) for chunked reading
// - XOR is its own inverse: encrypting twice gives the original
// - BufWriter improves performance for many small writes
//
// Example:
// Original bytes: [72, 101, 108, 108, 111] ("Hello")
// With key=42:    [98, 79, 70, 70, 69]
// Run tests with: cargo test --bin practice_binary_handler

pub fn xor_encrypt_file(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    key: u8,
) -> std::io::Result<u64> {
    todo!("Implement XOR encryption for binary files with buffered I/O")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufWriter, Read, Write};
    use std::path::Path;

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_xor_encrypt_file() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("test_binary.bin");
        let encrypted_path = temp_dir.join("test_encrypted.bin");
        let decrypted_path = temp_dir.join("test_decrypted.bin");

        // Create test binary file
        let original_data = b"Hello, World! This is a binary file test.";
        create_test_file(&input_path, original_data).unwrap();

        // Encrypt the file
        let bytes_processed = xor_encrypt_file(&input_path, &encrypted_path, 42).unwrap();
        assert_eq!(bytes_processed, original_data.len() as u64);

        // Verify encryption changed the content
        let encrypted_data = fs::read(&encrypted_path).unwrap();
        assert_ne!(encrypted_data, original_data);

        // Decrypt (XOR with same key)
        xor_encrypt_file(&encrypted_path, &decrypted_path, 42).unwrap();

        // Verify decryption restored original
        let decrypted_data = fs::read(&decrypted_path).unwrap();
        assert_eq!(decrypted_data, original_data);

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(encrypted_path).ok();
        fs::remove_file(decrypted_path).ok();
    }

    #[test]
    fn test_empty_binary_file() {
        let temp_dir = std::env::temp_dir();
        let empty_file = temp_dir.join("empty_binary.bin");
        let output = temp_dir.join("empty_encrypted.bin");

        create_test_file(&empty_file, b"").unwrap();

        let bytes_processed = xor_encrypt_file(&empty_file, &output, 42).unwrap();
        assert_eq!(bytes_processed, 0);

        let output_data = fs::read(&output).unwrap();
        assert_eq!(output_data.len(), 0);

        // Cleanup
        fs::remove_file(empty_file).ok();
        fs::remove_file(output).ok();
    }

    #[test]
    fn test_large_binary_file() {
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join("large_binary.bin");
        let output_path = temp_dir.join("large_encrypted.bin");

        // Create a larger test file (100KB)
        let large_data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        create_test_file(&input_path, &large_data).unwrap();

        let bytes_processed = xor_encrypt_file(&input_path, &output_path, 123).unwrap();
        assert_eq!(bytes_processed, 100_000);

        // Verify the output is different
        let encrypted_data = fs::read(&output_path).unwrap();
        assert_ne!(encrypted_data, large_data);
        assert_eq!(encrypted_data.len(), large_data.len());

        // Cleanup
        fs::remove_file(input_path).ok();
        fs::remove_file(output_path).ok();
    }
}
