use super::{NEXT, text};
use crate::daemon::Daemon;
use crate::store::private_dir;
use reiwand_library::Item;
use reiwand_providers::HttpFetch;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::Ordering;

impl Daemon {
    pub(crate) fn moving_file(&self, sha256: &str, ext: &str) -> PathBuf {
        self.store.cache.join("previews").join(format!("moving-{sha256}.{ext}"))
    }

    pub(super) fn fetch_animated(&self, spec: &str) -> Option<PathBuf> {
        let (repo_id, id, version, found) = self.offer_of(spec, true).ok()?;
        let found = found.filter(|p| ["gif", "webp"].contains(&p.ext.as_str()))?;
        let dest = self.moving_file(&found.sha256, &found.ext);
        if dest.is_file() {
            return Some(dest);
        }
        let dir = self.store.cache.join("previews");
        private_dir(&dir).ok()?;
        let download = dir.join(format!(".tmp-{}", NEXT.fetch_add(1, Ordering::Relaxed)));
        let fetch =
            crate::repos::pinned(self, HttpFetch::new(Arc::default()).ok()?.within(&self.bases(&repo_id)))
                .ok()?;
        let got = reiwand_format::animated(&self.store.data, &fetch, &repo_id, &id, &version, &download);
        let check =
            |data: Vec<u8>| reiwand_format::media::preview(true, &format!("p.{}", found.ext), &data).is_ok();
        let fine = got.unwrap_or(false) && std::fs::read(&download).is_ok_and(check);
        let placed = fine && std::fs::rename(&download, &dest).is_ok();
        std::fs::remove_file(&download).ok();
        placed.then(|| {
            self.thumbs.added(found.size, self.state().config.library.thumb_cache_mb);
            dest
        })
    }

    pub fn drop_animated(&self, uid: &str) {
        let prefix = format!("animated-{uid}-");
        let found = std::fs::read_dir(self.store.cache.join("previews")).into_iter().flatten().flatten();
        found
            .map(|e| e.path())
            .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&prefix)))
            .for_each(|p| drop(std::fs::remove_file(p)));
    }

    pub fn animated(&self, item: &Item, manifest: &reiwand_format::Manifest) -> Option<String> {
        let name = manifest.animated()?;
        let ext = std::path::Path::new(name).extension()?.to_str()?.to_ascii_lowercase();
        (ext == "gif" || ext == "webp").then_some(())?;
        let dir = self.store.cache.join("previews");
        let file = dir.join(format!("animated-{}-{}.{ext}", item.uid, item.mtime));
        if !file.is_file() {
            let data = reiwand_format::open(&item.path, &reiwand_format::Limits::default())
                .ok()?
                .read_file(name)
                .ok()?;
            private_dir(&dir).ok()?;
            let part = file.with_extension("part");
            let len = data.len() as u64;
            std::fs::write(&part, data).and_then(|()| std::fs::rename(&part, &file)).ok()?;
            let prefix = format!("animated-{}-", item.uid);
            let older = std::fs::read_dir(&dir).into_iter().flatten().flatten().map(|e| e.path());
            older
                .filter(|p| {
                    *p != file && p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&prefix))
                })
                .for_each(|p| drop(std::fs::remove_file(p)));
            self.thumbs.added(len, self.state().config.library.thumb_cache_mb);
        }
        Some(text(file))
    }
}
