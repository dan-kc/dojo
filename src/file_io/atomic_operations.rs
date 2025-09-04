// Exercise: Atomic File Operations
// Task: Implement a safe file replacement that writes to a temporary file first,
// then atomically replaces the original. This prevents data loss if the write fails.
//
// Hints:
// - Create a temp file with a unique name (e.g., add .tmp suffix)
// - Write all data to the temp file first
// - Use fs::rename() for atomic replacement (on same filesystem)
// - Clean up temp file if operation fails
//
// Requirements:
// - The function should apply a transformation function to the content
// - If any error occurs, the original file must remain unchanged
// - The temp file should be in the same directory as the target
// Run tests with: cargo test --bin practice_atomic_operations

pub fn safe_file_replace<F>(file_path: &std::path::Path, transform: F) -> std::io::Result<()>
where
    F: FnOnce(String) -> std::io::Result<String>,
{
    todo!("Implement atomic file replacement with transformation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, Read, Write};
    use std::path::Path;

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_safe_file_replace() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_atomic.txt");

        // Create initial file
        create_test_file(&file_path, b"original content").unwrap();

        // Define transformation
        let transform =
            |content: String| -> io::Result<String> { Ok(content.to_uppercase() + " - MODIFIED") };

        // Perform safe replacement
        safe_file_replace(&file_path, transform).unwrap();

        // Verify content was transformed
        let result = fs::read_to_string(&file_path).unwrap();
        assert_eq!(result, "ORIGINAL CONTENT - MODIFIED");

        // Test error case - transformation fails
        let failing_transform = |_: String| -> io::Result<String> {
            Err(io::Error::new(io::ErrorKind::Other, "Transform failed"))
        };

        let result = safe_file_replace(&file_path, failing_transform);
        assert!(result.is_err());

        // Verify original content is preserved after failure
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "ORIGINAL CONTENT - MODIFIED");

        // Cleanup
        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_safe_replace_with_unicode() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_unicode.txt");

        // Create file with unicode content
        create_test_file(&file_path, "Hello 世界 🦀".as_bytes()).unwrap();

        let transform =
            |content: String| -> io::Result<String> { Ok(content.replace("Hello", "Goodbye")) };

        safe_file_replace(&file_path, transform).unwrap();

        let result = fs::read_to_string(&file_path).unwrap();
        assert_eq!(result, "Goodbye 世界 🦀");

        // Cleanup
        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_safe_replace_non_existent_file() {
        let file_path = std::env::temp_dir().join("non_existent_file.txt");

        // Make sure file doesn't exist
        fs::remove_file(&file_path).ok();

        let transform = |content: String| -> io::Result<String> { Ok(content.to_uppercase()) };

        let result = safe_file_replace(&file_path, transform);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_replace_permission_error_simulation() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_permission.txt");

        create_test_file(&file_path, b"test content").unwrap();

        // Simulate error during write
        let transform = |content: String| -> io::Result<String> {
            if content.contains("test") {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Simulated permission error",
                ))
            } else {
                Ok(content)
            }
        };

        let result = safe_file_replace(&file_path, transform);
        assert!(result.is_err());

        // Verify original file is unchanged
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "test content");

        // Verify no temp files are left behind
        let parent = file_path.parent().unwrap();
        for entry in fs::read_dir(parent).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path != file_path {
                assert!(!path.to_string_lossy().contains(".tmp"));
            }
        }

        // Cleanup
        fs::remove_file(file_path).ok();
    }
}
