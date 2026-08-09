//! `report.md`/`state.json`/`manifest.json` were each written via a plain `std::fs::write`,
//! which truncates the destination file in place — a reader (or a second `codereview` run
//! sharing the same `--out` directory, e.g. two CI jobs racing) could observe a half-written
//! file mid-write, and a process that dies partway through leaves a truncated, corrupt file
//! behind instead of either the old content or the new content intact.

use anyhow::{Context, Result};
use std::path::Path;

/// Writes `contents` to `path` atomically: writes to a temp file in the same directory (so the
/// rename below is same-filesystem, and therefore atomic on every platform this project
/// targets — a cross-filesystem rename isn't), then renames over the target. A reader never
/// observes a partial file; on any failure, whatever was already at `path` is left untouched.
pub(crate) fn atomic_write(path: &Path, contents: &[u8]) -> Result<()> {
    let dir = path.parent().filter(|p| !p.as_os_str().is_empty());
    let dir = dir.unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("output");
    // The pid + a nanosecond timestamp keeps two concurrent runs targeting the same --out
    // directory from colliding on the same temp file name mid-write — collision there would
    // reopen the exact race this function exists to close, just one file name over.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = dir.join(format!(".{file_name}.tmp.{}.{nanos}", std::process::id()));

    std::fs::write(&tmp_path, contents)
        .with_context(|| format!("failed to write temp file {}", tmp_path.display()))?;
    std::fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "failed to move {} into place at {}",
            tmp_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_creates_a_new_file_with_the_given_contents() {
        let dir = std::env::temp_dir().join("codereview-loop-atomic-write-new-file-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("out.txt");

        atomic_write(&path, b"hello").unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn atomic_write_fully_replaces_existing_content_not_just_the_overlapping_prefix() {
        // Guards specifically against the failure mode a non-atomic in-place write doesn't
        // have to worry about accidentally reintroducing: a rename-based replace must swap the
        // whole file, not merge/append with what was there before.
        let dir = std::env::temp_dir().join("codereview-loop-atomic-write-replace-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("out.txt");

        atomic_write(&path, b"a very long original line of content").unwrap();
        atomic_write(&path, b"short").unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "short");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn atomic_write_leaves_no_leftover_temp_file_behind() {
        let dir = std::env::temp_dir().join("codereview-loop-atomic-write-no-leftover-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("out.txt");

        atomic_write(&path, b"content").unwrap();

        let entries: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        assert_eq!(
            entries,
            vec![std::ffi::OsString::from("out.txt")],
            "the temp file must be renamed away, not left alongside the real one: {entries:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
