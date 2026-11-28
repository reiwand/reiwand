//! The web helper seen from `scene render`: what it says, and the frames of a page on a frame clock.
use crate::told::{fill, told, tr};
use anyhow::{Context, Result, bail};
use image::RgbaImage;
use reiwand_web::{FromWeb, Link, ToWeb};
use std::os::unix::fs::FileExt;
use std::time::{Duration, Instant};

const STEP: Duration = Duration::from_secs(10);
const QUIET: Duration = Duration::from_millis(100);

pub struct Run {
    pub link: Link,
    pub child: std::process::Child,
    pub ring: Option<std::fs::File>,
    pub loaded: bool,
}

impl Run {
    /// Takes what the helper says until `until`; returns the last frame, or at once `Stepped` when `stepped`.
    pub fn pump(&mut self, until: Instant, stepped: bool) -> Result<Option<RgbaImage>> {
        let mut last = None;
        while Instant::now() < until {
            match self.link.recv::<FromWeb>(false) {
                Ok(Some((FromWeb::Pixels { view, slot, size, stride, .. }, fds))) => {
                    fds.into_iter()
                        .next()
                        .into_iter()
                        .for_each(|fd| self.ring = Some(std::fs::File::from(fd)));
                    let mut bgra = vec![0u8; stride as usize * size.1 as usize];
                    let ring = self
                        .ring
                        .as_ref()
                        .with_context(|| tr(told("the web helper gave no frame buffer")))?;
                    let offset = u64::from(slot) * bgra.len() as u64;
                    ring.read_exact_at(&mut bgra, offset)?;
                    self.link.send(&ToWeb::Freed { view, slot }, &[])?;
                    last = Some(picture(&bgra, size, stride));
                    if !stepped {
                        return Ok(last);
                    }
                }
                Ok(Some((FromWeb::Frame { view, buffer }, _))) => {
                    self.link.send(&ToWeb::Released { view, buffer }, &[])?
                }
                Ok(Some((FromWeb::Stepped { .. }, _))) if stepped => return Ok(Some(RgbaImage::new(1, 1))),
                Ok(Some((FromWeb::Loaded { .. }, _))) => self.loaded = true,
                Ok(Some((FromWeb::Console { line, .. } | FromWeb::Said(line), _))) => {
                    eprintln!("{}", fill(told("page: %1"), &[&line]))
                }
                Ok(Some((FromWeb::Failed(why) | FromWeb::Unavailable(why), _))) => bail!(why),
                Ok(Some((FromWeb::Crashed { .. } | FromWeb::Broken { .. }, _))) => {
                    bail!(tr(told("the page crashed")))
                }
                Ok(Some(_)) => {}
                Ok(None) => bail!(tr(told("the web helper stopped"))),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(2))
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(last.filter(|_| !stepped))
    }

    /// Moves the page to `at` seconds and draws it once that time is painted.
    pub fn frame(&mut self, at: f64, pointer: Option<(f64, f64)>) -> Result<RgbaImage> {
        if pointer.is_some() {
            self.link.send(&ToWeb::Pointer { view: 1, at: pointer }, &[])?;
        }
        self.link.send(&ToWeb::Step { view: 1, at }, &[])?;
        if self.pump(Instant::now() + STEP, true)?.is_none() {
            bail!(fill(told("the page did not draw second %1"), &[&at]));
        }
        while self.pump(Instant::now() + QUIET, false)?.is_some() {}
        self.link.send(&ToWeb::Shot { view: 1 }, &[])?;
        self.pump(Instant::now() + STEP, false)?.with_context(|| tr(told("the web helper gave no frame")))
    }
}

impl Drop for Run {
    fn drop(&mut self) {
        let _ = self.link.send(&ToWeb::Quit, &[]);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn picture(bgra: &[u8], (w, h): (u32, u32), stride: u32) -> RgbaImage {
    RgbaImage::from_fn(w, h, |x, y| {
        let i = (y * stride + x * 4) as usize;
        image::Rgba([bgra[i + 2], bgra[i + 1], bgra[i], 255])
    })
}
