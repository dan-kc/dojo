# Directory Synchronization Solution

## Solution

```rust
pub fn sync_directories(
    source: &std::path::Path,
    destination: &std::path::Path,
    delete_orphans: bool,
) -> std::io::Result<(usize, usize)> {
    use std::collections::HashSet;
    use std::fs;
    use std::time::SystemTime;

    let mut copied = 0;
    let mut deleted = 0;

    // Ensure destination directory exists
    fs::create_dir_all(destination)?;

    // Track all files in source for orphan detection
    let mut source_files = HashSet::new();

    // First pass: copy files from source to destination
    sync_directory_recursive(source, destination, &mut source_files, &mut copied)?;

    // Second pass: delete orphans if requested
    if delete_orphans {
        delete_orphans_recursive(destination, &source_files, &mut deleted)?;
    }

    Ok((copied, deleted))
}

fn sync_directory_recursive(
    source_dir: &std::path::Path,
    dest_dir: &std::path::Path,
    source_files: &mut HashSet<std::path::PathBuf>,
    copied: &mut usize,
) -> std::io::Result<()> {
    use std::fs;
    use std::time::SystemTime;

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(source_dir)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Path error"))?;
        let dest_path = dest_dir.join(&relative_path);

        if source_path.is_dir() {
            // Create destination directory if it doesn't exist
            fs::create_dir_all(&dest_path)?;
            // Recursively sync subdirectory
            sync_directory_recursive(&source_path, &dest_path, source_files, copied)?;
        } else {
            // Track this file as existing in source
            source_files.insert(relative_path.to_path_buf());

            let should_copy = if !dest_path.exists() {
                true // File doesn't exist in destination
            } else {
                // Compare modification times
                let source_modified = source_path.metadata()?.modified()?;
                let dest_modified = dest_path.metadata()?.modified()?;
                source_modified > dest_modified
            };

            if should_copy {
                // Ensure parent directory exists
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&source_path, &dest_path)?;
                *copied += 1;
            }
        }
    }

    Ok(())
}

fn delete_orphans_recursive(
    dest_dir: &std::path::Path,
    source_files: &HashSet<std::path::PathBuf>,
    deleted: &mut usize,
) -> std::io::Result<()> {
    use std::fs;

    let entries: Vec<_> = fs::read_dir(dest_dir)?.collect::<Result<Vec<_>, _>>()?;
    
    for entry in entries {
        let dest_path = entry.path();
        
        if dest_path.is_dir() {
            // Recursively check subdirectories
            delete_orphans_recursive(&dest_path, source_files, deleted)?;
            
            // Remove directory if it's empty after orphan deletion
            if fs::read_dir(&dest_path)?.next().is_none() {
                fs::remove_dir(&dest_path)?;
            }
        } else {
            // Check if this file exists in source
            let relative_path = dest_path.strip_prefix(dest_dir)
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Path error"))?;
            
            if !source_files.contains(relative_path) {
                fs::remove_file(&dest_path)?;
                *deleted += 1;
            }
        }
    }

    Ok(())
}
```

## Key Concepts Explained

- **Two-Pass Algorithm**: First pass copies/updates files, second pass removes orphans
- **Relative Path Handling**: Using `strip_prefix()` to work with relative paths for cross-directory operations
- **Time Comparison**: Comparing `SystemTime` values to determine if files need updating
- **Directory Creation**: Using `create_dir_all()` to ensure parent directories exist
- **Orphan Detection**: Using `HashSet` to efficiently track which files exist in the source
- **Recursive Deletion**: Carefully removing orphaned files and empty directories