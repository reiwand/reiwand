use super::counted::allocations;
use super::{Net, SIZE, TIMES};
use image::{ImageBuffer, Luma, RgbaImage};
use reiwand_core::Fit;
use reiwand_render::{GpuImage, Layer, Relief, Renderer, Target};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::time::Instant;

const STEADY: u32 = 10;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures").join(name)
}

fn layer<'a>(image: &'a GpuImage, relief: Option<Relief<'a>>) -> Layer<'a> {
    Layer {
        image: Some(image),
        fit: Fit::Fill,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief,
        flip: [false; 2],
        grade: None,
    }
}

fn shot(r: &Renderer, target: &mut Target, pixels: &mut Vec<u8>, layers: &[Layer<'_>]) -> RgbaImage {
    r.render_layers(target, layers, pixels).unwrap();
    let mut rgba = pixels.clone();
    rgba.as_chunks_mut::<4>().0.iter_mut().for_each(|p| p.swap(0, 2));
    RgbaImage::from_raw(SIZE.0, SIZE.1, rgba).unwrap()
}

fn steady(r: &Renderer, net: &mut Net, name: &str, layers: &[Layer<'_>]) {
    let (mut target, mut pixels) = (r.create_target(SIZE.0, SIZE.1).unwrap(), Vec::new());
    (0..3).for_each(|_| r.render_layers(&mut target, layers, &mut pixels).unwrap());
    let started = Instant::now();
    let ((), made) = allocations(|| {
        (0..STEADY).for_each(|_| r.render_layers(&mut target, layers, &mut pixels).unwrap());
    });
    let ms = started.elapsed().as_secs_f64() * 1e3 / f64::from(STEADY);
    let counters =
        json!({ "allocations": made.div_ceil(u64::from(STEADY)) as f64, "ms": (ms * 100.0).round() / 100.0 });
    net.counters(name, counters);
}

fn upload(r: &Renderer, image: &RgbaImage) -> GpuImage {
    r.upload_rgba(image.as_raw(), image.width(), image.height(), image.width() * 4).unwrap()
}

fn pictures(r: &Renderer, net: &mut Net) {
    let photo = image::open(fixture("bands-640x180.png")).unwrap().into_rgba8();
    let image = upload(r, &photo);
    let (mut target, mut pixels) = (r.create_target(SIZE.0, SIZE.1).unwrap(), Vec::new());
    net.picture("picture", &[shot(r, &mut target, &mut pixels, &[layer(&image, None)])]);
    steady(r, net, "picture", &[layer(&image, None)]);
    let (w, h) = photo.dimensions();
    let field =
        ImageBuffer::from_fn(w / 4, h / 4, |x, y| Luma([((x * 4 + y * 2) * 65535 / (w + h / 2)) as u16]));
    let depth = r.upload_layers(std::slice::from_ref(&photo), &[field], &[0.0]).unwrap();
    let relief = |shift| Some(Relief { depth: &depth, shift, focus: 0.5, steps: 8, overscan: 1.05 });
    let shifts = [[0.0, 0.0], [0.03, 0.0], [-0.02, 0.02]];
    let frames: Vec<_> =
        shifts.iter().map(|&s| shot(r, &mut target, &mut pixels, &[layer(&image, relief(s))])).collect();
    net.picture("depth", &frames);
    steady(r, net, "depth", &[layer(&image, relief(shifts[1]))]);
}

fn video(r: &Renderer, net: &mut Net) {
    let file = fixture("quadrants-64x32.webm");
    let mut stills = Vec::new();
    for t in TIMES {
        match reiwand_video::still_at(&file, Some(t)) {
            Ok(still) => stills.push(RgbaImage::from_raw(still.width, still.height, still.rgba).unwrap()),
            Err(e) if std::env::var_os("REIWAND_REQUIRE_VIDEO").is_none() => {
                eprintln!("skipping the video wallpaper: {e}");
                return net.skip("video");
            }
            Err(e) => return net.fail(format!("video: not decoded: {e}")),
        }
    }
    let (mut target, mut pixels) = (r.create_target(SIZE.0, SIZE.1).unwrap(), Vec::new());
    let images: Vec<_> = stills.iter().map(|s| upload(r, s)).collect();
    let frames: Vec<_> =
        images.iter().map(|i| shot(r, &mut target, &mut pixels, &[layer(i, None)])).collect();
    net.picture("video", &frames);
    steady(r, net, "video", &[layer(&images[0], None)]);
}

pub fn check(r: &Renderer, net: &mut Net) {
    pictures(r, net);
    video(r, net);
}
