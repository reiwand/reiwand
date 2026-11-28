use image::{Rgba, RgbaImage};
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};

const OFF: u8 = 24;
const OFF_SHARE: f64 = 0.004;
const MEAN: f64 = 1.5;
const GROWTH: f64 = 0.10;
const SLOWER: f64 = 8.0;
const SLOWER_MS: f64 = 20.0;
const OTHER_DRIVER: f64 = 2.0;
const OTHER_DRIVER_MORE: f64 = 50.0;

pub struct Net {
    bless: bool,
    out: PathBuf,
    kept: Map<String, Value>,
    measured: Map<String, Value>,
    same_driver: bool,
    failures: Vec<String>,
}

fn here(file: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/net").join(file)
}

fn sheet(frames: &[RgbaImage]) -> RgbaImage {
    let (w, h) = frames[0].dimensions();
    let mut sheet = RgbaImage::new(w, h * frames.len() as u32);
    for (n, frame) in frames.iter().enumerate() {
        image::imageops::replace(&mut sheet, frame, 0, i64::from(n as u32 * h));
    }
    sheet
}

fn compare(got: &RgbaImage, expected: &RgbaImage) -> Option<String> {
    if got.dimensions() != expected.dimensions() {
        return Some(format!(
            "the size is {:?}, the reference is {:?}",
            got.dimensions(),
            expected.dimensions()
        ));
    }
    let pairs = got.pixels().zip(expected.pixels());
    let off = pairs.filter(|(a, b)| a.0.iter().zip(b.0).any(|(x, y)| x.abs_diff(y) > OFF)).count();
    let raw = got.as_raw().iter().zip(expected.as_raw());
    let mean = raw.map(|(a, b)| f64::from(a.abs_diff(*b))).sum::<f64>() / got.as_raw().len() as f64;
    let allowed = (f64::from(got.width() * got.height()) * OFF_SHARE) as usize;
    (off > allowed || mean > MEAN)
        .then(|| format!("{off} pixels differ (at most {allowed}), mean difference {mean:.2}"))
}

fn difference(got: &RgbaImage, expected: &RgbaImage) -> RgbaImage {
    RgbaImage::from_fn(got.width(), got.height(), |x, y| {
        let (a, b) = (got.get_pixel(x, y).0, expected.get_pixel(x, y).0);
        let d = |c: usize| a[c].abs_diff(b[c]).saturating_mul(4);
        Rgba([d(0), d(1), d(2), 255])
    })
}

impl Net {
    pub fn new(adapter: &str) -> Net {
        let out = std::env::var_os("REIWAND_NET_OUT")
            .map_or_else(|| Path::new(env!("CARGO_TARGET_TMPDIR")).join("net"), PathBuf::from);
        let _ = std::fs::remove_dir_all(&out);
        let kept = std::fs::read(here("counters.json")).ok().and_then(|b| serde_json::from_slice(&b).ok());
        let kept: Map<String, Value> = kept.unwrap_or_default();
        let bless = std::env::var_os("REIWAND_BLESS").is_some();
        let same_driver = kept.get("adapter").and_then(Value::as_str) == Some(adapter);
        if !same_driver && !bless {
            eprintln!(
                "the references were stored with another driver: allocations only fail when they double"
            );
        }
        let measured = Map::from_iter([("adapter".to_string(), json!(adapter))]);
        Net { bless, out, kept, measured, same_driver, failures: Vec::new() }
    }

    pub fn fail(&mut self, failure: String) {
        self.failures.push(failure);
    }

    pub fn skip(&mut self, name: &str) {
        if let Some(kept) = self.kept.get(name).cloned() {
            self.measured.insert(name.into(), kept);
        }
    }

    pub fn picture(&mut self, name: &str, frames: &[RgbaImage]) {
        let (got, path) = (sheet(frames), here(&format!("pictures/{name}.png")));
        if self.bless {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            return got.save(&path).unwrap();
        }
        let Ok(expected) = image::open(&path).map(|i| i.into_rgba8()) else {
            return self.fail(format!(
                "{name}: no reference picture; refresh them with scripts/regression.sh --bless"
            ));
        };
        let Some(why) = compare(&got, &expected) else { return };
        std::fs::create_dir_all(&self.out).unwrap();
        got.save(self.out.join(format!("{name}-got.png"))).unwrap();
        expected.save(self.out.join(format!("{name}-expected.png"))).unwrap();
        if got.dimensions() == expected.dimensions() {
            difference(&got, &expected).save(self.out.join(format!("{name}-difference.png"))).unwrap();
        }
        self.fail(format!("{name}: {why}; pictures in {}", self.out.display()));
    }

    pub fn counters(&mut self, name: &str, counters: Value) {
        let kept = self.kept.get(name).cloned().unwrap_or(Value::Null);
        for (key, got) in counters.as_object().unwrap() {
            let got = got.as_f64().unwrap();
            let Some(was) = kept[key].as_f64() else {
                if !self.bless {
                    self.fail(format!(
                        "{name}: no stored {key}; refresh them with scripts/regression.sh --bless"
                    ));
                }
                continue;
            };
            let limit = match key.as_str() {
                "ms" => was * SLOWER + SLOWER_MS,
                "allocations" if !self.same_driver => was * OTHER_DRIVER + OTHER_DRIVER_MORE,
                _ => was + (was * GROWTH).max(1.0),
            };
            if got > limit && !self.bless {
                self.fail(format!("{name}: {key} is {got}, stored {was}, at most {limit:.1}"));
            }
        }
        self.measured.insert(name.into(), counters);
    }

    pub fn finish(self) {
        if self.bless {
            let text = serde_json::to_string_pretty(&Value::Object(self.measured)).unwrap();
            return std::fs::write(here("counters.json"), text + "\n").unwrap();
        }
        let mut failures = self.failures;
        let gone = self.kept.keys().filter(|k| !self.measured.contains_key(*k));
        failures.extend(gone.map(|k| format!("{k}: stored but no longer measured")));
        let summary = json!(self.measured.keys().collect::<Vec<_>>());
        assert!(failures.is_empty(), "the regression net failed:\n{}", failures.join("\n"));
        eprintln!("regression net held for {summary}");
    }
}
