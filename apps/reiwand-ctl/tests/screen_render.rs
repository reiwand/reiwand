#[path = "whole/mod.rs"]
mod whole;

use std::path::Path;
use std::process::Command;

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

fn has_gpu() -> bool {
    reiwand_render::Renderer::shared().is_ok() || std::env::var_os("REIWAND_REQUIRE_GPU").is_some()
}

fn scene(home: &Path, fit: &str) -> std::path::PathBuf {
    let folder = home.join("wide");
    assert!(ctl(home).args(["scene", "new"]).arg(&folder).output().unwrap().status.success());
    let mut doc: serde_json::Value =
        serde_json::from_slice(&std::fs::read(folder.join("scene.json")).unwrap()).unwrap();
    doc["canvas"]["size"] = serde_json::json!([64, 48]);
    doc["canvas"]["fit"] = serde_json::json!(fit);
    doc["canvas"]["background"] = serde_json::json!("#0000ff");
    doc["layers"] =
        serde_json::json!([{ "blend_space": "canvas", "id": "red", "type": "solid", "color": "#ff0000" }]);
    std::fs::write(folder.join("scene.json"), whole::scene(&doc.to_string())).unwrap();
    folder
}

fn render(home: &Path, folder: &Path, args: &[&str]) -> image::RgbaImage {
    let out = home.join("out.png");
    let done = ctl(home)
        .args(["scene", "render"])
        .arg(folder)
        .arg("-o")
        .arg(&out)
        .args(["--time-of-day", "12:00"])
        .args(args)
        .output()
        .unwrap();
    assert!(done.status.success(), "{}", String::from_utf8_lossy(&done.stderr));
    image::open(&out).unwrap().to_rgba8()
}

fn is(pixel: &image::Rgba<u8>, want: [u8; 3]) -> bool {
    (0..3).all(|i| pixel.0[i].abs_diff(want[i]) < 24)
}

#[test]
fn a_fill_canvas_is_drawn_at_the_size_of_the_screen() {
    if !has_gpu() {
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let folder = scene(home.path(), "fill");
    let image = render(home.path(), &folder, &["--screen", "160x90"]);
    assert_eq!(image.dimensions(), (160, 90));
    assert!(
        [(0, 0), (159, 0), (80, 45), (0, 89), (159, 89)]
            .iter()
            .all(|&(x, y)| is(image.get_pixel(x, y), [255, 0, 0]))
    );
}

#[test]
fn a_fit_canvas_leaves_bars_of_the_background_colour() {
    if !has_gpu() {
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let folder = scene(home.path(), "fit");
    let image = render(home.path(), &folder, &["--screen", "160x90"]);
    assert_eq!(image.dimensions(), (160, 90));
    assert!(is(image.get_pixel(5, 45), [0, 0, 255]), "{:?}", image.get_pixel(5, 45));
    assert!(is(image.get_pixel(154, 45), [0, 0, 255]));
    assert!(is(image.get_pixel(80, 45), [255, 0, 0]));
    assert!(is(image.get_pixel(30, 2), [255, 0, 0]));
}

#[test]
fn the_screen_and_the_size_cannot_be_given_together() {
    let home = tempfile::tempdir().unwrap();
    let out = ctl(home.path())
        .args(["scene", "render", "x", "-o", "y.png", "--screen", "160x90", "--size", "64"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("cannot be used with"));
    let bad =
        ctl(home.path()).args(["scene", "render", "x", "-o", "y.png", "--screen", "wide"]).output().unwrap();
    assert!(String::from_utf8_lossy(&bad.stderr).contains("WIDTHxHEIGHT"));
}
