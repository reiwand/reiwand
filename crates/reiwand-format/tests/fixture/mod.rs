#![allow(dead_code)]

use reiwand_format::{
    Content, Fetch, FileFetch, Keypair, Kind, Manifest, Origin, Previews, Rating, SCHEMA, Signer, Text,
    keygen,
};
use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod repo;

pub const NOW: i64 = 1_800_000_000;
pub const DAY: i64 = 86_400;
pub const DAWN: &str = "org.example.dawn";
pub const DUSK: &str = "org.example.dusk";

pub struct Repo {
    pub tmp: TempDir,
    pub dir: PathBuf,
    pub data: PathBuf,
    pub root: Keypair,
    pub online: Keypair,
    pub author: Keypair,
}

fn picture(format: image::ImageFormat) -> Vec<u8> {
    let mut out = io::Cursor::new(Vec::new());
    image::RgbImage::from_pixel(64, 36, [20, 30, 40].into()).write_to(&mut out, format).unwrap();
    out.into_inner()
}

pub fn key() -> Keypair {
    keygen(None).unwrap()
}

pub fn signer(key: &Keypair) -> Signer {
    Signer::new(&key.secret, None).unwrap()
}

pub fn pair(key: &Keypair) -> (&str, &str) {
    (&key.fingerprint, key.public.lines().nth(1).unwrap())
}

pub fn file_url(dir: &Path) -> String {
    format!("file://{}/", dir.display())
}

pub const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x40\0\0\0\x24\x08\x02\0\0\0";

pub fn manifest(id: &str, version: &str) -> Manifest {
    Manifest {
        schema: SCHEMA.into(),
        content: Some(Content { rating: Rating::General, descriptors: Vec::new() }),
        photosensitive: Some(false),
        origin: Some(Origin::default()),
        id: id.into(),
        version: version.into(),
        title: Text::Plain(format!("Sky at {}", id.rsplit('.').next().unwrap())),
        description: "A calm gradient".into(),
        authors: vec!["Example Studio".into()],
        license: "CC0-1.0".into(),
        kind: Kind::Image,
        entry: "image.png".into(),
        previews: Some(Previews { thumbnail: Some("preview.png".into()), animated: None }),
        tags: vec!["sky".into()],
        ..Default::default()
    }
}

pub fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for item in fs::read_dir(from).unwrap() {
        let item = item.unwrap();
        let target = to.join(item.file_name());
        if item.file_type().unwrap().is_dir() {
            copy_dir(&item.path(), &target);
        } else {
            fs::copy(item.path(), target).unwrap();
        }
    }
}

pub fn files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> =
        fs::read_dir(dir).unwrap().map(|i| i.unwrap().file_name().into_string().unwrap()).collect();
    names.sort();
    names
}

#[derive(Default)]
pub struct Counting(pub RefCell<Vec<String>>);

impl Fetch for Counting {
    fn get(&self, base: &str, path: &str, max: u64) -> io::Result<Option<Vec<u8>>> {
        self.0.borrow_mut().push(path.into());
        FileFetch.get(base, path, max)
    }
}

impl Counting {
    pub fn count(&self, prefix: &str) -> usize {
        self.0.borrow().iter().filter(|p| p.starts_with(prefix)).count()
    }
}
