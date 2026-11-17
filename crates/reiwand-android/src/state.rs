use crate::{Error, said};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::{Duration, Instant};

pub(crate) static CANCEL: AtomicBool = AtomicBool::new(false);

pub(crate) async fn pause(seconds: u64) -> Result<(), Error> {
    let until = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < until {
        if CANCEL.load(Relaxed) {
            return Err(said("android.stopped", "stopped", &[]));
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    Ok(())
}
