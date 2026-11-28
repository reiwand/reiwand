mod common;

use reiwand_format::{Kind, Limits, build, open};
use std::fs;

const SCENE: &str = r#"{"schema": "reiwand.scene/2", "canvas": {"size": [64, 32]},
    "assets": {"f": {"path": "assets/big.ttf"}},
    "layers": [{ "blend_space": "canvas", "id": "t", "type": "text", "text": "Hi", "font": "f", "size": 16}]}"#;

#[test]
fn a_font_above_its_limit_is_not_read() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("src"), tmp.path().join("font.wpak"));
    fs::create_dir_all(src.join("assets")).unwrap();
    fs::write(src.join("manifest.json"), common::manifest(Kind::Scene).to_json().unwrap()).unwrap();
    fs::write(src.join("preview.webp"), b"RIFF\0\0\0\0WEBP").unwrap();
    fs::write(src.join("scene.json"), common::whole(SCENE)).unwrap();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut noise = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed.to_le_bytes()
    };
    let font: Vec<u8> = (0..(reiwand_scene::MAX_FONT / 8 + 1)).flat_map(|_| noise()).collect();
    fs::write(src.join("assets/big.ttf"), &font).unwrap();
    build(&src, &out, &Limits::default()).unwrap();
    let mut package = open(&out, &Limits::default()).unwrap();
    assert!(package.read_asset("assets/big.ttf").unwrap().is_empty());
    assert_eq!(package.read_file("assets/big.ttf").unwrap().len(), font.len());
    assert_eq!(package.read_asset("scene.json").unwrap(), package.read_file("scene.json").unwrap());
}
