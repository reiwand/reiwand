use std::path::Path;

const MAX_ENTRIES: usize = 100_000;
const KEPT: [&str; 7] = [
    "Local Storage",
    "Session Storage",
    "IndexedDB",
    "Service Worker",
    "File System",
    "databases",
    "Cookies",
];

fn kept(name: &std::ffi::OsStr) -> bool {
    KEPT.iter().any(|kept| name.to_str().is_some_and(|name| name.starts_with(kept)))
}

pub fn within(dir: &Path, kib: u32) -> bool {
    let allowed = reiwand_format::kept_allowance(kib);
    let mut todo: Vec<_> = std::fs::read_dir(dir.join("data"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| kept(&entry.file_name()))
        .map(|entry| entry.path())
        .collect();
    let (mut used, mut seen) = (0u64, 0usize);
    while let Some(dir) = todo.pop() {
        let entries = if dir.is_dir() { std::fs::read_dir(&dir).ok() } else { None };
        for entry in entries.into_iter().flatten().flatten() {
            seen += 1;
            match entry.metadata() {
                Ok(meta) if meta.is_dir() => todo.push(entry.path()),
                Ok(meta) => used += meta.len(),
                Err(_) => {}
            }
            if used > allowed || seen > MAX_ENTRIES {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn kept_data_stays_near_what_the_page_declared() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join("data/origin");
        std::fs::create_dir_all(&data).unwrap();
        std::fs::write(data.join("small"), vec![0; 100 << 10]).unwrap();
        assert!(super::within(dir.path(), 64));
        std::fs::write(data.join("large"), vec![0; 2 << 20]).unwrap();
        assert!(!super::within(dir.path(), 64));
        assert!(super::within(dir.path(), 2048));
    }

    #[test]
    fn engine_caches_do_not_count_as_kept_data() {
        let dir = tempfile::tempdir().unwrap();
        let cache = dir.path().join("data/GPUCache");
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(cache.join("data_1"), vec![0; 4 << 20]).unwrap();
        std::fs::write(dir.path().join("data/Visited Links"), vec![0; 2 << 20]).unwrap();
        assert!(super::within(dir.path(), 4));
        let kept = dir.path().join("data/Local Storage/leveldb");
        std::fs::create_dir_all(&kept).unwrap();
        std::fs::write(kept.join("000003.log"), vec![0; 3 << 20]).unwrap();
        assert!(!super::within(dir.path(), 4));
    }
}
