mod common;

use reiwand_format::{Kind, Limits, build, open};
use std::fs;

const SCENE: &str = r#"{"schema": "reiwand.scene/2", "canvas": {"size": [64, 32]},
    "assets": {"song": {"path": "assets/song.mp3"}},
    "layers": [{ "blend_space": "canvas", "id": "song", "type": "sound", "sound": "song", "stream": true}]}"#;

#[test]
fn a_streamed_mp3_behind_a_tag_larger_than_the_edge_read_is_accepted() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("src"), tmp.path().join("song.wpak"));
    fs::create_dir_all(src.join("assets")).unwrap();
    fs::write(src.join("manifest.json"), common::manifest(Kind::Scene).to_json().unwrap()).unwrap();
    fs::write(src.join("preview.webp"), b"RIFF\0\0\0\0WEBP").unwrap();
    fs::write(src.join("scene.json"), common::whole(SCENE)).unwrap();
    let tag: u32 = 1_200_000;
    let syncsafe = [21, 14, 7, 0].map(|shift| (tag >> shift & 0x7f) as u8);
    let frame = [&[0xff, 0xfb, 0x90, 0x00][..], &[0; 413]].concat();
    let song = [&b"ID3\x04\0\0"[..], &syncsafe, &vec![0; tag as usize], &frame.repeat(4000)].concat();
    fs::write(src.join("assets/song.mp3"), song).unwrap();
    reiwand_format::inspect(&src).unwrap();
    build(&src, &out, &Limits::default()).unwrap();
    assert_eq!(open(&out, &Limits::default()).unwrap().scene().unwrap().sounds().count(), 1);
}
