use std::sync::mpsc::{self, Receiver, TryRecvError};

pub(super) enum Later<T> {
    Waiting(Receiver<Option<T>>),
    Ready(Option<T>),
}

impl<T: Send + 'static> Later<T> {
    pub(super) fn spawn(work: impl FnOnce() -> Option<T> + Send + 'static) -> Self {
        let (sender, receiver) = mpsc::channel();
        let started = std::thread::Builder::new().name("reiwand-open".into()).spawn(move || {
            let _ = sender.send(work());
        });
        match started {
            Ok(_) => Later::Waiting(receiver),
            Err(e) => {
                tracing::warn!("cannot start the opener: {e}");
                Later::Ready(None)
            }
        }
    }

    pub(super) fn arrived(&mut self) -> bool {
        let Later::Waiting(receiver) = self else { return false };
        match receiver.try_recv() {
            Ok(value) => *self = Later::Ready(value),
            Err(TryRecvError::Empty) => return false,
            Err(TryRecvError::Disconnected) => *self = Later::Ready(None),
        }
        true
    }

    pub(super) fn get(&self) -> Option<&T> {
        match self {
            Later::Ready(value) => value.as_ref(),
            Later::Waiting(_) => None,
        }
    }

    pub(super) fn take(self) -> Option<T> {
        match self {
            Later::Ready(value) => value,
            Later::Waiting(_) => None,
        }
    }

    pub(super) fn waiting(&self) -> bool {
        matches!(self, Later::Waiting(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn a_stalled_open_never_blocks_the_caller() {
        let (release, stalled) = mpsc::channel::<()>();
        let start = Instant::now();
        let mut later = Later::spawn(move || stalled.recv().ok().map(|_| 7));
        assert!(start.elapsed() < Duration::from_millis(200));
        assert!(later.waiting() && !later.arrived() && later.get().is_none());
        release.send(()).unwrap();
        while !later.arrived() {
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(later.get(), Some(&7));
    }

    #[test]
    fn a_dropped_waiter_releases_the_late_value_off_the_caller() {
        let (release, stalled) = mpsc::channel::<()>();
        let (dropped, seen) = mpsc::channel::<()>();
        struct Mark(mpsc::Sender<()>);
        impl Drop for Mark {
            fn drop(&mut self) {
                let _ = self.0.send(());
            }
        }
        let later = Later::spawn(move || stalled.recv().ok().map(|_| Mark(dropped)));
        drop(later);
        release.send(()).unwrap();
        assert!(seen.recv_timeout(Duration::from_secs(5)).is_ok());
    }
}
