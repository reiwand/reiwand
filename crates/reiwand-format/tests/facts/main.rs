#[path = "../common/mod.rs"]
mod common;

use common::{archive, image, package, v2};
use image::ImageFormat;
use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, RgbaImage};
use reiwand_format::{
    Error, Input, Kind, Limits, Manifest, Previews, build, features, fill, media, open, read,
};
use std::fs;
use std::io::Cursor;
use std::path::Path;

mod checked;

const SCENE: &str = r##"{"schema": "reiwand.scene/1", "canvas": {"size": [3840, 1600]},
    "layers": [{"id": "glow", "type": "solid", "color": "#336699", "opacity": {"audio": "bass"}}]}"##;

fn source(dir: &Path, manifest: &Manifest) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("manifest.json"), manifest.to_json().unwrap()).unwrap();
    fs::write(dir.join("scene.json"), SCENE).unwrap();
    fs::write(dir.join("preview.png"), image(320, 180, ImageFormat::Png)).unwrap();
}

fn gif(frames: u32, centis: u32) -> Vec<u8> {
    let mut out = Vec::new();
    let frame = |_| Frame::from_parts(RgbaImage::new(8, 4), 0, 0, Delay::from_numer_denom_ms(centis * 10, 1));
    GifEncoder::new(&mut out).encode_frames((0..frames).map(frame)).unwrap();
    out
}

fn animated_webp(millis: [u32; 2]) -> Vec<u8> {
    let chunk = |name: &[u8], data: Vec<u8>| [name, &(data.len() as u32).to_le_bytes(), &data].concat();
    let anmf = |ms: u32| chunk(b"ANMF", [vec![0; 12], ms.to_le_bytes()[..3].to_vec(), vec![0]].concat());
    let vp8x = chunk(
        b"VP8X",
        [vec![2, 0, 0, 0], 99u32.to_le_bytes()[..3].to_vec(), 49u32.to_le_bytes()[..3].to_vec()].concat(),
    );
    let body = [b"WEBP".to_vec(), vp8x, anmf(millis[0]), anmf(millis[1])].concat();
    [b"RIFF".to_vec(), (body.len() as u32).to_le_bytes().to_vec(), body].concat()
}
