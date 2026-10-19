# file_io/directory_sync

Reference implementation. Read after attempting the exercise.

```rust
pub fn sync_directories(
    source: &std::path::Path,
    destination: &std::path::Path,
    delete_orphans: bool,
) -> std::io::Result<(usize, usize)> {
    fn files(
        root: &std::path::Path,
        dir: &std::path::Path,
        out: &mut std::collections::BTreeSet<std::path::PathBuf>,
    ) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let kind = entry.file_type()?;
            if kind.is_dir() {
                files(root, &entry.path(), out)?;
            } else if kind.is_file() {
                out.insert(entry.path().strip_prefix(root).unwrap().to_owned());
            }
        }
        Ok(())
    }
    let mut source_files = std::collections::BTreeSet::new();
    files(source, source, &mut source_files)?;
    std::fs::create_dir_all(destination)?;
    let mut copied = 0;
    for relative in &source_files {
        let src = source.join(relative);
        let dest = destination.join(relative);
        let data = std::fs::read(&src)?;
        let equal = match std::fs::read(&dest) {
            Ok(old) => old == data,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => return Err(e),
        };
        if !equal {
            std::fs::create_dir_all(dest.parent().unwrap())?;
            std::fs::copy(src, dest)?;
            copied += 1;
        }
    }
    let mut deleted = 0;
    if delete_orphans {
        let mut dest_files = std::collections::BTreeSet::new();
        files(destination, destination, &mut dest_files)?;
        for relative in dest_files.difference(&source_files) {
            std::fs::remove_file(destination.join(relative))?;
            deleted += 1;
        }
    }
    Ok((copied, deleted))
}
```
