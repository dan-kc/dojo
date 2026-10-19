# file_io/metadata_analyzer

Reference implementation. Read after attempting the exercise.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FileStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub largest_file: Option<(std::path::PathBuf, u64)>,
    pub extensions: std::collections::HashMap<String, usize>,
}
pub fn analyze_directory(dir_path: &std::path::Path) -> std::io::Result<FileStats> {
    fn visit(dir: &std::path::Path, s: &mut FileStats) -> std::io::Result<()> {
        for e in std::fs::read_dir(dir)? {
            let e = e?;
            let kind = e.file_type()?;
            let path = e.path();
            if kind.is_dir() {
                s.total_dirs += 1;
                visit(&path, s)?;
            } else if kind.is_file() {
                let size = e.metadata()?.len();
                s.total_files += 1;
                s.total_size += size;
                *s.extensions
                    .entry(
                        path.extension()
                            .and_then(|x| x.to_str())
                            .unwrap_or("")
                            .to_owned(),
                    )
                    .or_default() += 1;
                if s.largest_file
                    .as_ref()
                    .is_none_or(|(p, n)| size > *n || size == *n && path < *p)
                {
                    s.largest_file = Some((path, size));
                }
            }
        }
        Ok(())
    }
    let mut stats = FileStats {
        total_files: 0,
        total_dirs: 0,
        total_size: 0,
        largest_file: None,
        extensions: std::collections::HashMap::new(),
    };
    visit(dir_path, &mut stats)?;
    Ok(stats)
}
```
