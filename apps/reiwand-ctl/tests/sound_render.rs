#[path = "whole/mod.rs"]
mod whole;

use std::path::{Path, PathBuf};
use std::process::Command;

fn has_gpu() -> bool {
    reiwand_render::Renderer::shared().is_ok() || std::env::var_os("REIWAND_REQUIRE_GPU").is_some()
}

fn ctl(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_reiwand-ctl"));
    command
        .env("XDG_STATE_HOME", home.join("state"))
        .env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("REIWAND_PRIVATE_BUS_ONLY", "1")
        .env("LANG", "C.UTF-8");
    command
}

fn wav(path: &Path, rate: u32, samples: &[i16]) {
    let data = (samples.len() * 2) as u32;
    let mut bytes = b"RIFF".to_vec();
    bytes.extend((36 + data).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16u32.to_le_bytes());
    bytes.extend([1u16, 1].iter().flat_map(|v| v.to_le_bytes()));
    bytes.extend(rate.to_le_bytes());
    bytes.extend((rate * 2).to_le_bytes());
    bytes.extend([2u16, 16].iter().flat_map(|v| v.to_le_bytes()));
    bytes.extend(b"data");
    bytes.extend(data.to_le_bytes());
    bytes.extend(samples.iter().flat_map(|s| s.to_le_bytes()));
    std::fs::write(path, bytes).unwrap();
}

fn kicks(rate: u32) -> Vec<i16> {
    let kick = |i: usize| {
        let t = i as f64 / f64::from(rate);
        let wave = (std::f64::consts::TAU * 60.0 * t).sin() * (-(t % 0.5) * 12.0).exp();
        if t >= 0.3 { (wave * 30_000.0) as i16 } else { 0 }
    };
    (0..2 * rate as usize).map(kick).collect()
}

fn lamp(home: &Path) -> PathBuf {
    let folder = home.join("lamp");
    assert!(ctl(home).args(["scene", "new"]).arg(&folder).output().unwrap().status.success());
    let mut doc: serde_json::Value =
        serde_json::from_slice(&std::fs::read(folder.join("scene.json")).unwrap()).unwrap();
    doc["canvas"]["size"] = serde_json::json!([32, 18]);
    doc["canvas"]["background"] = serde_json::json!("#000000");
    let bass = serde_json::json!({ "audio": "bass", "kind": "relative", "unit": "db" });
    doc["layers"] = serde_json::json!([{ "blend_space": "canvas", "id": "lamp", "type": "solid",
        "color": "#ffffff", "opacity": bass }]);
    std::fs::write(folder.join("scene.json"), whole::scene(&doc.to_string())).unwrap();
    folder
}

fn render(home: &Path, folder: &Path, name: &str, audio: Option<&Path>) -> Vec<Vec<u8>> {
    let mut command = ctl(home);
    command.args(["scene", "render"]).arg(folder).arg("-o").arg(home.join(name));
    command.args(["--time-of-day", "12:00", "--frames", "24", "--fps", "30"]);
    if let Some(audio) = audio {
        command.arg("--audio").arg(audio);
    }
    let done = command.output().unwrap();
    assert!(done.status.success(), "{}", String::from_utf8_lossy(&done.stderr));
    let written = String::from_utf8(done.stdout).unwrap();
    written.lines().map(|f| image::open(f).unwrap().to_rgba8().into_raw()).collect()
}

#[test]
fn a_sound_file_moves_the_scene_the_same_way_every_time_and_silence_stays_without_one() {
    if !has_gpu() {
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let folder = lamp(home.path());
    let track = home.path().join("kicks.wav");
    wav(&track, 44_100, &kicks(44_100));
    let quiet = render(home.path(), &folder, "quiet.png", None);
    let heard = render(home.path(), &folder, "heard.png", Some(&track));
    let again = render(home.path(), &folder, "again.png", Some(&track));
    assert_eq!(quiet.len(), 24);
    assert!(
        quiet.iter().flatten().enumerate().all(|(i, &v)| i % 4 == 3 || v == 0),
        "silence without --audio"
    );
    assert_ne!(quiet, heard, "the kicks light the lamp");
    assert_eq!(heard[..8], quiet[..8], "the track is silent until 0.3 s");
    assert_eq!(heard, again, "the same bytes twice");
    let missing = ctl(home.path())
        .args(["scene", "render"])
        .arg(&folder)
        .arg("-o")
        .arg(home.path().join("x.png"))
        .arg("--audio")
        .arg(home.path().join("nothing.wav"))
        .output()
        .unwrap();
    assert!(!missing.status.success() && String::from_utf8_lossy(&missing.stderr).contains("nothing.wav"));
}
