use super::{block_on, sync_error};
use crate::compositor::lock;
use crate::error::{Error, Result};
use crate::jobs::Ctl;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const CHECK: Duration = Duration::from_millis(250);
/// How long an upload may hand nothing to the connection, and wait for the answer once all of it
/// is sent, before the network counts as gone: the server answers a whole file within seconds.
pub(super) const STALLED: (Duration, Duration) = (Duration::from_secs(60), Duration::from_secs(300));
const GIVE_BACK: Duration = Duration::from_secs(10);

/// Runs one request of an upload until it ends or `stop` is set (the job was cancelled), so Cancel
/// stops it at once instead of after the request's own limit.
pub(super) fn stoppable<T>(
    stop: Arc<AtomicBool>,
    work: impl Future<Output = reiwand_account::Result<T>>,
) -> Result<T> {
    block_on(async move {
        let cancelled = async {
            while !stop.load(Ordering::Relaxed) {
                tokio::time::sleep(CHECK).await;
            }
        };
        tokio::select! {
            done = work => done.map_err(sync_error),
            () = cancelled => Err(Error::Cancelled),
        }
    })
}

/// Waits for the upload of `size` bytes counted in `sent`, telling `progress` how far it is, and
/// fails when nothing moves for `limits.0` while sending, or `limits.1` once all is sent: a network
/// that went silent keeps the connection open.
pub(super) fn watched<T>(
    stop: Arc<AtomicBool>,
    progress: impl Fn(u64, u64),
    (size, sent): (u64, &Arc<AtomicU64>),
    limits: (Duration, Duration),
    work: impl Future<Output = reiwand_account::Result<T>>,
) -> Result<T> {
    let sent = sent.clone();
    let watch = async {
        let (mut last, mut still) = (u64::MAX, Duration::ZERO);
        loop {
            tokio::time::sleep(CHECK).await;
            let now = sent.load(Ordering::Relaxed).min(size);
            (last, still) = if now == last { (last, still + CHECK) } else { (now, Duration::ZERO) };
            progress(now, size);
            if still >= if now < size { limits.0 } else { limits.1 } {
                return;
            }
        }
    };
    stoppable(stop, async {
        tokio::select! {
            done = work => done,
            () = watch => Err(reiwand_account::Error::Network(crate::words::upload_stalled())),
        }
    })
}

/// `watched` for a job: its Cancel and its progress.
pub(super) fn sent_by<T>(
    ctl: &Ctl,
    size: u64,
    sent: &Arc<AtomicU64>,
    work: impl Future<Output = reiwand_account::Result<T>>,
) -> Result<T> {
    watched(ctl.flag(), |done, total| ctl.progress(done, total), (size, sent), STALLED, work)
}

/// The uploads this computer opened and could not give back (the server was unreachable), by the
/// server's address, kept in `uploads-abandoned.json` of the daemon's data so a restart keeps them.
type Abandoned = BTreeMap<String, BTreeSet<String>>;
static FILE: Mutex<()> = Mutex::new(());

fn kept(file: &Path) -> Abandoned {
    reiwand_core::stored::read(file).ok().flatten().unwrap_or_default()
}

/// A file of another format, or damaged, is left as it is and not used, like the other data files.
fn readable(file: &Path) -> bool {
    reiwand_core::stored::read::<Abandoned>(file)
        .inspect_err(|e| tracing::warn!("the abandoned uploads are not kept: {e}"))
        .is_ok()
}

fn keep(file: &Path, change: impl FnOnce(&mut Abandoned)) {
    let _one = lock(&FILE);
    if !readable(file) {
        return;
    }
    let mut list = kept(file);
    change(&mut list);
    list.retain(|_, ids| !ids.is_empty());
    let written = (|| -> std::io::Result<()> {
        let dir = file.parent().unwrap_or(Path::new("."));
        let mut out = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
        out.write_all(&reiwand_core::stored::text(&list)?)?;
        out.persist(file).map(drop).map_err(|e| e.error)
    })();
    if let Err(e) = written {
        tracing::warn!("cannot keep the abandoned uploads in {}: {e}", file.display());
    }
}

fn give_back(api: &reiwand_account::Api, id: &str) -> bool {
    let path = format!("/api/v1/uploads/{id}");
    let gone =
        block_on(async { tokio::time::timeout(GIVE_BACK, api.slow(reqwest::Method::DELETE, &path)).await });
    // a server that no longer has it (expired, finished elsewhere) is as good as given back
    matches!(gone, Ok(Ok(_))) || matches!(gone, Ok(Err(reiwand_account::Error::Server { status: 404, .. })))
}

/// Gives back the upload a failed or cancelled job had opened, so it holds neither one of the two
/// open uploads the server allows nor its bytes of the quota. It runs aside so the job ends at once;
/// without a network it is kept in `file` and tried again before the next upload to that server.
pub(super) fn abandon(api: &reiwand_account::Api, id: &str, file: PathBuf) {
    let (api, id) = (api.clone(), id.to_owned());
    std::thread::spawn(move || {
        if !give_back(&api, &id) {
            keep(&file, |list| {
                list.entry(api.base.clone()).or_default().insert(id);
            });
        }
    });
}

/// Before a new upload: gives back the uploads abandoned on this server while it was unreachable.
pub(super) fn retry_abandoned(api: &reiwand_account::Api, file: &Path) {
    let mine = kept(file).remove(&api.base).unwrap_or_default();
    for id in mine {
        if give_back(api, &id) {
            keep(file, |list| {
                list.get_mut(&api.base).map(|ids| ids.remove(&id));
            });
        }
    }
}

#[cfg(test)]
#[path = "sending_tests.rs"]
mod tests;
