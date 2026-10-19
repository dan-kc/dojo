// cargo test --lib file_io::metadata_analyzer::

//! Recursively summarize regular files and directories below dir_path (exclude the root from
//! the directory count). Ignore symlinks and special files. Sum byte sizes and count case-sensitive
//! extensions without the dot; absent/non-UTF-8 extensions use the empty key. The largest file
//! uses the lexicographically smallest full PathBuf to break size ties. Propagate filesystem errors.

#[derive(Debug, Clone, PartialEq)]
pub struct FileStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub largest_file: Option<(std::path::PathBuf, u64)>,
    pub extensions: std::collections::HashMap<String, usize>,
}
pub fn analyze_directory(dir_path: &std::path::Path) -> std::io::Result<FileStats> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_analyze_directory() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path().join("test_analyze_dir");

        // Create test directory structure
        fs::create_dir_all(&temp_dir).unwrap();
        fs::create_dir_all(temp_dir.join("subdir")).unwrap();

        create_test_file(&temp_dir.join("file1.txt"), b"Hello").unwrap();
        create_test_file(&temp_dir.join("file2.txt"), b"World!!!").unwrap();
        create_test_file(&temp_dir.join("file3.rs"), b"fn main() {}").unwrap();
        create_test_file(&temp_dir.join("subdir/file4.txt"), b"Nested").unwrap();
        create_test_file(&temp_dir.join("no_extension"), b"Test").unwrap();

        // Analyze directory
        let stats = analyze_directory(&temp_dir).unwrap();

        // Verify statistics
        assert_eq!(stats.total_files, 5);
        assert_eq!(stats.total_dirs, 1); // subdir
        assert_eq!(stats.total_size, 35); // Total bytes of all files

        // Check largest file
        let (largest_path, largest_size) = stats.largest_file.unwrap();
        assert_eq!(largest_size, 12); // "fn main() {}" is 12 bytes
        assert!(largest_path.ends_with("file3.rs"));

        // Check extensions
        assert_eq!(stats.extensions.get("txt"), Some(&3));
        assert_eq!(stats.extensions.get("rs"), Some(&1));
        assert_eq!(stats.extensions.get(""), Some(&1)); // no_extension file

        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn test_empty_directory() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
        let empty_dir = temp_dir.join("empty_dir_test");
        fs::create_dir_all(&empty_dir).unwrap();

        let stats = analyze_directory(&empty_dir).unwrap();
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.total_dirs, 0);
        assert_eq!(stats.total_size, 0);
        assert!(stats.largest_file.is_none());
        assert!(stats.extensions.is_empty());

        // Cleanup
        fs::remove_dir(empty_dir).ok();
    }

    #[test]
    fn test_nested_directories() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path().join("nested_test");

        // Create deeply nested structure
        fs::create_dir_all(temp_dir.join("a/b/c/d")).unwrap();

        create_test_file(&temp_dir.join("root.txt"), b"root").unwrap();
        create_test_file(&temp_dir.join("a/level1.txt"), b"level1").unwrap();
        create_test_file(&temp_dir.join("a/b/level2.rs"), b"level2").unwrap();
        create_test_file(&temp_dir.join("a/b/c/level3.txt"), b"level3").unwrap();
        create_test_file(&temp_dir.join("a/b/c/d/level4.txt"), b"level4").unwrap();

        let stats = analyze_directory(&temp_dir).unwrap();

        assert_eq!(stats.total_files, 5);
        assert_eq!(stats.total_dirs, 4); // a, b, c, d
        assert_eq!(stats.extensions.get("txt"), Some(&4));
        assert_eq!(stats.extensions.get("rs"), Some(&1));

        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }

    #[test]
    fn test_non_existent_directory() {
        let sandbox = tempfile::tempdir().unwrap();
        let missing = sandbox.path().join("missing");
        let non_existent = missing.as_path();
        let result = analyze_directory(non_existent);
        assert!(result.is_err());
    }
}
