# Atomic File Operations Solution

## Solution

```rust
pub fn safe_file_replace<F>(file_path: &std::path::Path, transform: F) -> std::io::Result<()>
where
    F: FnOnce(String) -> std::io::Result<String>,
{
    use std::fs;
    use std::io::Write;

    // Read the original content
    let original_content = fs::read_to_string(file_path)?;

    // Apply the transformation
    let transformed_content = transform(original_content)?;

    // Create a temporary file in the same directory for atomic replacement
    let temp_path = {
        let mut temp = file_path.as_os_str().to_owned();
        temp.push(".tmp");
        std::path::PathBuf::from(temp)
    };

    // Write to temporary file first
    {
        let mut temp_file = fs::File::create(&temp_path)?;
        temp_file.write_all(transformed_content.as_bytes())?;
        temp_file.flush()?; // Ensure data is written before rename
    } // temp_file is closed here due to scope

    // Atomically replace the original file
    fs::rename(&temp_path, file_path)?;

    Ok(())
}
```

## Key Concepts Explained

- **Atomic Operations**: Using `fs::rename()` ensures the replacement is atomic on most filesystems
- **Temporary File Strategy**: Writing to a temp file first prevents data corruption if the operation fails
- **Generic Function Parameters**: The `F: FnOnce(String) -> std::io::Result<String>` constraint allows flexible transformations
- **Scope-based Resource Management**: The temporary file is automatically closed when it goes out of scope
- **Error Safety**: If any operation fails, the original file remains unchanged
- **Path Manipulation**: Creating the temp filename by appending `.tmp` to the original path