use super::*;
use std::time::Instant;

fn forever() -> impl Future<Output = reiwand_account::Result<()>> {
    std::future::pending()
}

#[test]
fn a_cancelled_upload_stops_at_once_instead_of_waiting_for_its_request() {
    let stop = Arc::new(AtomicBool::new(false));
    let setter = stop.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        setter.store(true, Ordering::Relaxed);
    });
    let started = Instant::now();
    assert!(matches!(stoppable(stop, forever()), Err(Error::Cancelled)));
    assert!(started.elapsed() < Duration::from_secs(2));
}

#[test]
fn an_upload_that_sends_nothing_fails_with_the_reason_and_reports_how_far_it_got() {
    let (stop, sent) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicU64::new(40)));
    let seen = Mutex::new((0, 0));
    let limits = (Duration::from_millis(600), Duration::from_secs(60));
    let failed = watched(stop, |d, t| *lock(&seen) = (d, t), (100, &sent), limits, forever());
    let message = failed.err().map(|e| e.to_string()).unwrap_or_default();
    assert!(message.contains("nothing reached it for a minute"), "{message}");
    assert_eq!(*lock(&seen), (40, 100));
}

#[test]
fn an_upload_that_keeps_moving_is_not_called_stalled() {
    let (stop, sent) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicU64::new(0)));
    let feeder = sent.clone();
    let work = async move {
        for _ in 0..8 {
            tokio::time::sleep(Duration::from_millis(200)).await;
            feeder.fetch_add(10, Ordering::Relaxed);
        }
        Ok(7)
    };
    let limits = (Duration::from_millis(600), Duration::from_millis(600));
    assert_eq!(watched(stop, |_, _| {}, (100, &sent), limits, work).ok(), Some(7));
}

#[test]
fn abandoned_uploads_are_kept_by_server_and_dropped_when_given_back() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("uploads-abandoned.json");
    keep(&file, |list| {
        list.entry("https://a.example".into()).or_default().insert("u1".into());
    });
    keep(&file, |list| {
        list.entry("https://b.example".into()).or_default().insert("u2".into());
    });
    assert_eq!(kept(&file).len(), 2);
    keep(&file, |list| {
        list.get_mut("https://a.example").map(|ids| ids.remove("u1"));
    });
    let left = kept(&file);
    assert!(!left.contains_key("https://a.example") && left["https://b.example"].contains("u2"));
}

#[test]
fn a_damaged_list_of_abandoned_uploads_is_never_written_over() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("uploads-abandoned.json");
    std::fs::write(&file, b"{not json").unwrap();
    keep(&file, |list| {
        list.entry("https://a.example".into()).or_default().insert("u1".into());
    });
    assert_eq!(std::fs::read(&file).unwrap(), b"{not json");
    assert!(kept(&file).is_empty());
}
