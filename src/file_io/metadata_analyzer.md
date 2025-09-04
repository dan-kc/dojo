# File Metadata and Directory Operations Solution

## Solution

```rust
pub fn analyze_directory(dir_path: &std::path::Path) -> std::io::Result<FileStats> {
    use std::collections::HashMap;
    use std::fs;

    let mut stats = FileStats {
        total_files: 0,
        total_dirs: 0,
        total_size: 0,
        largest_file: None,
        extensions: HashMap::new(),
    };

    // Use a recursive helper function to traverse the directory tree
    analyze_directory_recursive(dir_path, &mut stats)?;
    Ok(stats)
}

fn analyze_directory_recursive(dir_path: &std::path::Path, stats: &mut FileStats) -> std::io::Result<()> {
    use std::fs;

    // Read directory entries
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            stats.total_dirs += 1;
            // Recursively analyze subdirectories
            analyze_directory_recursive(&path, stats)?;
        } else if metadata.is_file() {
            stats.total_files += 1;
            let file_size = metadata.len();
            stats.total_size += file_size;

            // Track largest file
            if let Some((_, current_largest)) = &stats.largest_file {
                if file_size > *current_largest {
                    stats.largest_file = Some((path.clone(), file_size));
                }
            } else {
                stats.largest_file = Some((path.clone(), file_size));
            }

            // Count file extensions
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_string();
            
            *stats.extensions.entry(extension).or_insert(0) += 1;
        }
    }

    Ok(())
}
```

## Key Concepts Explained

- **Recursive Directory Traversal**: Using a helper function for clean recursive processing
- **Metadata Access**: `entry.metadata()` provides file size, type, and other filesystem information
- **HashMap Operations**: Using `entry().or_insert()` for efficient counting with default values
- **Option Handling**: Gracefully handling files without extensions using `unwrap_or("")`
- **Path Manipulation**: Using `path.extension()` to extract file extensions safely
- **Mutable Borrowing**: Passing mutable references to accumulate state across recursive calls