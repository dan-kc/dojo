# file_io/atomic_operations

Reference implementation. Read after attempting the exercise.

```rust
pub fn safe_file_replace<F>(file_path: &std::path::Path, transform: F) -> std::io::Result<()>
where
    F: FnOnce(String) -> std::io::Result<String>,
{
    use std::io::Write;
    let text = std::fs::read_to_string(file_path)?;
    let permissions = std::fs::metadata(file_path)?.permissions();
    let replacement = transform(text)?;
    let parent = file_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    tmp.write_all(replacement.as_bytes())?;
    tmp.as_file().set_permissions(permissions)?;
    tmp.as_file().sync_all()?;
    tmp.persist(file_path).map_err(|e| e.error)?;
    Ok(())
}
```
