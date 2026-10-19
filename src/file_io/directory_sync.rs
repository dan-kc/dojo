// cargo test --lib file_io::directory_sync::

//! Recursively synchronize regular files from source to destination, copying missing files or
//! files whose contents differ. Create required directories. With delete_orphans, delete destination
//! regular files absent from source; empty directories may remain. Return (copied files, deleted files).
//! Do not follow symlinks. Roots are disjoint, source exists, and neither tree contains symlinks or
//! file/directory type conflicts (caller preconditions). Propagate I/O errors. Modification timestamps
//! are insufficient to decide equality. Reading one whole file at a time is acceptable.

pub fn sync_directories(
    source: &std::path::Path,
    destination: &std::path::Path,
    delete_orphans: bool,
) -> std::io::Result<(usize, usize)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;

    fn create_test_file(path: &Path, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_sync_directories() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
        let source_dir = temp_dir.join("sync_source");
        let dest_dir = temp_dir.join("sync_dest");

        // Create source directory structure
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();

        // Create source files
        create_test_file(&source_dir.join("file1.txt"), b"Source 1").unwrap();
        create_test_file(&source_dir.join("file2.txt"), b"Source 2").unwrap();

        // Create destination with one outdated file and one orphan
        create_test_file(&dest_dir.join("file1.txt"), b"Old").unwrap();
        create_test_file(&dest_dir.join("orphan.txt"), b"Will be deleted").unwrap();

        // Make source file1.txt newer by touching it

        create_test_file(&source_dir.join("file1.txt"), b"Source 1 New").unwrap();

        // Sync without deleting orphans
        let (copied, deleted) = sync_directories(&source_dir, &dest_dir, false).unwrap();
        assert_eq!(copied, 2); // file1.txt (updated) and file2.txt (new)
        assert_eq!(deleted, 0);
        assert!(dest_dir.join("orphan.txt").exists());

        // Sync with orphan deletion
        let (copied, deleted) = sync_directories(&source_dir, &dest_dir, true).unwrap();
        assert_eq!(copied, 0); // Nothing new to copy
        assert_eq!(deleted, 1); // orphan.txt deleted
        assert!(!dest_dir.join("orphan.txt").exists());

        // Verify synced content
        let content1 = fs::read_to_string(dest_dir.join("file1.txt")).unwrap();
        assert_eq!(content1, "Source 1 New");

        // Cleanup
        fs::remove_dir_all(source_dir).ok();
        fs::remove_dir_all(dest_dir).ok();
    }

    #[test]
    fn test_sync_empty_directories() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
        let source_dir = temp_dir.join("empty_source");
        let dest_dir = temp_dir.join("empty_dest");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();

        let (copied, deleted) = sync_directories(&source_dir, &dest_dir, false).unwrap();
        assert_eq!(copied, 0);
        assert_eq!(deleted, 0);

        // Cleanup
        fs::remove_dir_all(source_dir).ok();
        fs::remove_dir_all(dest_dir).ok();
    }

    #[test]
    fn test_sync_with_subdirectories() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
        let source_dir = temp_dir.join("nested_source");
        let dest_dir = temp_dir.join("nested_dest");

        // Create nested structure
        fs::create_dir_all(source_dir.join("subdir")).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();

        create_test_file(&source_dir.join("root.txt"), b"root").unwrap();
        create_test_file(&source_dir.join("subdir/nested.txt"), b"nested").unwrap();

        let (copied, _deleted) = sync_directories(&source_dir, &dest_dir, false).unwrap();
        assert_eq!(copied, 2);

        // Verify nested structure was created
        assert!(dest_dir.join("subdir").exists());
        assert!(dest_dir.join("subdir/nested.txt").exists());

        // Cleanup
        fs::remove_dir_all(source_dir).ok();
        fs::remove_dir_all(dest_dir).ok();
    }

    #[test]
    fn test_sync_delete_orphans_with_subdirs() {
        let sandbox = tempfile::tempdir().unwrap();
        let temp_dir = sandbox.path();
        let source_dir = temp_dir.join("orphan_source");
        let dest_dir = temp_dir.join("orphan_dest");

        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(dest_dir.join("orphan_subdir")).unwrap();

        // Create orphan files in destination
        create_test_file(&dest_dir.join("orphan1.txt"), b"orphan1").unwrap();
        create_test_file(&dest_dir.join("orphan_subdir/orphan2.txt"), b"orphan2").unwrap();

        // Create one file in source
        create_test_file(&source_dir.join("keep.txt"), b"keep").unwrap();

        let (copied, deleted) = sync_directories(&source_dir, &dest_dir, true).unwrap();
        assert_eq!(copied, 1); // keep.txt
        assert_eq!(deleted, 2); // both orphans

        assert!(dest_dir.join("keep.txt").exists());
        assert!(!dest_dir.join("orphan1.txt").exists());
        assert!(!dest_dir.join("orphan_subdir/orphan2.txt").exists());

        // Cleanup
        fs::remove_dir_all(source_dir).ok();
        fs::remove_dir_all(dest_dir).ok();
    }

    #[test]
    fn test_sync_non_existent_source() {
        let sandbox = tempfile::tempdir().unwrap();
        let source = sandbox.path().join("non_existent_src");
        let dest = sandbox.path().join("test_dest");

        fs::create_dir_all(&dest).unwrap();

        let result = sync_directories(&source, &dest, false);
        assert!(result.is_err());

        // Cleanup
        fs::remove_dir_all(dest).ok();
    }
}
