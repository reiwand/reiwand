use reiwand_video::{Cut, Options, Player};
use std::path::Path;
use std::sync::{Mutex, mpsc};
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/magenta-64x32.webm");

#[test]
fn dropping_a_stuck_player_never_blocks_its_caller() {
    let (tx, rx) = mpsc::channel();
    let (release, stuck) = mpsc::channel::<()>();
    let stuck = Mutex::new(stuck);
    let options = Options {
        max_size: (64, 32),
        max_fps: 60,
        volume: 0.0,
        muted: true,
        cut: Cut::default(),
        zero_copy: false,
    };
    let opened = Player::open(Path::new(FIXTURE), options, move |_| {
        let _ = tx.send(());
        let _ = stuck.lock().map(|stuck| stuck.recv());
    });
    let Some(player) = opened.ok().filter(|_| rx.recv_timeout(Duration::from_secs(10)).is_ok()) else {
        assert!(std::env::var_os("REIWAND_REQUIRE_VIDEO").is_none(), "video playback required");
        return;
    };
    let start = Instant::now();
    drop(player);
    let took = start.elapsed();
    drop(release);
    assert!(took < Duration::from_millis(500), "dropping a stuck player blocked its caller for {took:?}");
}
