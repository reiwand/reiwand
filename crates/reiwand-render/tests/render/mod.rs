use image::{Rgba, RgbaImage};
use reiwand_core::Fit;
use reiwand_render::{GpuImage, Layer, Planes, Renderer, TargetLimits};

mod bright;
mod cost;
mod deep;
mod layers;
mod looks;
mod peak;
mod readback;

use crate::gpu::renderer;

fn layer(image: Option<&GpuImage>, fit: Fit, background: [u8; 3], opacity: f32) -> Layer<'_> {
    Layer { image, fit, background, opacity, uv: None, relief: None, flip: [false; 2], grade: None }
}

fn draw(r: &Renderer, image: Option<&RgbaImage>, fit: Fit, background: [u8; 3], size: (u32, u32)) -> Vec<u8> {
    let uploaded = image.map(|img| r.upload(img, fit, size).unwrap());
    let mut target = r.create_target(size.0, size.1).unwrap();
    let mut pixels = Vec::new();
    r.render_layers(&mut target, &[layer(uploaded.as_ref(), fit, background, 1.0)], &mut pixels).unwrap();
    pixels
}

fn rgb(pixels: &[u8], width: u32, x: u32, y: u32) -> [u8; 3] {
    let i = ((y * width + x) * 4) as usize;
    [pixels[i + 2], pixels[i + 1], pixels[i]]
}

fn close(a: [u8; 3], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2)
}
