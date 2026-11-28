#![cfg(all(feature = "capture", not(target_os = "android")))]

use reiwand_audio::Others;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::time::Duration;

const MARK: &str = "reiwand.owner";

/// Two seconds of a quiet tone, as a WAV file pw-cat plays.
fn tone() -> PathBuf {
    let (rate, frames) = (48_000u32, 96_000u32);
    let bytes = frames * 4;
    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + bytes).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&[1, 0, 2, 0]);
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 4).to_le_bytes());
    wav.extend_from_slice(&[4, 0, 16, 0]);
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&bytes.to_le_bytes());
    for i in 0..frames {
        let s = ((i as f32 * 0.05).sin() * 800.0) as i16;
        wav.extend_from_slice(&s.to_le_bytes());
        wav.extend_from_slice(&s.to_le_bytes());
    }
    let path = std::env::temp_dir().join(format!("reiwand-others-{}.wav", std::process::id()));
    std::fs::write(&path, wav).expect("the tone is written");
    path
}

fn play(file: &PathBuf, owner: &str) -> Option<Child> {
    let props = format!("{{ {MARK} = \"{owner}\" }}");
    Command::new("pw-cat").args(["--playback", "--properties", &props]).arg(file).spawn().ok()
}

/// Whether the watcher says another application plays within `wait`.
fn told(rx: &mpsc::Receiver<bool>, wait: Duration) -> bool {
    let end = std::time::Instant::now() + wait;
    while let Some(left) = end.checked_duration_since(std::time::Instant::now()) {
        match rx.recv_timeout(left) {
            Ok(true) => return true,
            Ok(false) => {}
            Err(_) => return false,
        }
    }
    false
}

#[test]
fn a_stream_with_this_process_mark_is_not_another_application() {
    let (tx, rx) = mpsc::channel();
    let owner: &'static str = Box::leak(format!("{}-test", std::process::id()).into_boxed_str());
    let Ok(_watch) = Others::watch(
        |_| false,
        (MARK, owner),
        move |playing| {
            let _ = tx.send(playing);
        },
    ) else {
        return eprintln!("skipped: no PipeWire");
    };
    let file = tone();
    // Another application first: without it heard, this computer cannot play a stream at all.
    let Some(mut other) = play(&file, "someone-else") else {
        return eprintln!("skipped: no pw-cat");
    };
    let heard = told(&rx, Duration::from_secs(4));
    other.kill().ok();
    other.wait().ok();
    if !heard {
        return eprintln!("skipped: no stream of pw-cat ran (no audio output?)");
    }
    while rx.recv_timeout(Duration::from_secs(3)) == Ok(true) {}
    let mut mine = play(&file, owner).expect("pw-cat started a moment ago");
    let counted = told(&rx, Duration::from_secs(3));
    mine.kill().ok();
    mine.wait().ok();
    std::fs::remove_file(&file).ok();
    assert!(!counted, "a stream that carries this process's mark counted as another application");
}
