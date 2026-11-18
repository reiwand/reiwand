#![allow(dead_code)]

use image::{ImageFormat, RgbImage};
use reiwand_format::{Content, FileEntry, Kind, MANIFEST, Manifest, Origin, Previews, Rating, SCHEMA};
use std::io::{Cursor, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

pub type Zip = ZipWriter<Cursor<Vec<u8>>>;

pub fn manifest(kind: Kind) -> Manifest {
    let entry = match kind {
        Kind::Image => "image.png",
        Kind::Video => "loop.webm",
        Kind::Scene => "scene.json",
        Kind::Web => "index.html",
        Kind::Preset => "preset.json",
        Kind::Assets => "assets.json",
    };
    Manifest {
        schema: SCHEMA.into(),
        content: Some(Content { rating: Rating::General, descriptors: Vec::new() }),
        photosensitive: Some(false),
        origin: Some(Origin::default()),
        id: "org.example.sunset".into(),
        version: "1.0.0".into(),
        title: "Sunset".into(),
        description: "A calm evening sky".into(),
        authors: vec!["Jane Doe".into()],
        license: "CC-BY-4.0".into(),
        kind,
        entry: entry.into(),
        tags: vec!["nature".into()],
        ..Default::default()
    }
}

pub fn v2(kind: Kind) -> Manifest {
    Manifest {
        previews: Some(Previews { thumbnail: Some("preview.png".into()), animated: None }),
        ..manifest(kind)
    }
}

pub fn image(width: u32, height: u32, format: ImageFormat) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    RgbImage::from_pixel(width, height, [40, 90, 160].into()).write_to(&mut out, format).unwrap();
    out.into_inner()
}

pub fn entry(path: &str, size: u64) -> FileEntry {
    FileEntry { path: path.into(), size, compressed_size: size, symlink: false }
}

pub fn files(manifest: &Manifest) -> Vec<FileEntry> {
    vec![entry(MANIFEST, 300), entry(&manifest.entry, 1000)]
}

pub fn zip(add: impl FnOnce(&mut Zip)) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    add(&mut zip);
    zip.finish().unwrap().into_inner()
}

pub fn whole(text: &str) -> String {
    reiwand_upgrade::fill::whole(&text.replace("reiwand.scene/1", "reiwand.scene/2"))
}

pub fn manifest_text(text: &str) -> String {
    reiwand_upgrade::manifests::whole(text)
}

pub fn manifest_of(value: serde_json::Value) -> Manifest {
    let text = manifest_text(&value.to_string());
    Manifest::from_json(text.as_bytes()).unwrap_or_else(|e| panic!("{e:?} {text}"))
}

pub fn member<T: serde::de::DeserializeOwned>(field: &str, value: serde_json::Value) -> T {
    let mut doc = serde_json::to_value(manifest(Kind::Scene)).unwrap();
    doc[field] = value;
    serde_json::from_value(serde_json::to_value(manifest_of(doc)).unwrap()[field].take()).unwrap()
}

pub fn properties(properties: serde_json::Value) -> serde_json::Value {
    let doc = serde_json::json!({ "schema": "reiwand.wpak/3", "id": "org.example.props", "version": "1.0.0",
        "title": "Props", "license": "MIT", "kind": "scene", "entry": "scene.json", "properties": properties });
    serde_json::from_str::<serde_json::Value>(&manifest_text(&doc.to_string())).unwrap()["properties"].take()
}

pub fn put(zip: &mut Zip, name: &str, data: &[u8]) {
    let done;
    let data = match std::str::from_utf8(data) {
        Ok(text) if name.ends_with("scene.json") => {
            done = whole(text);
            done.as_bytes()
        }
        Ok(text) if name == MANIFEST => {
            done = manifest_text(text);
            done.as_bytes()
        }
        _ => data,
    };
    if name.ends_with('/') {
        zip.add_directory(name, SimpleFileOptions::default()).unwrap();
    } else {
        zip.start_file(name, SimpleFileOptions::default()).unwrap();
        zip.write_all(data).unwrap();
    }
}

pub fn archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
    zip(|z| entries.iter().for_each(|(name, data)| put(z, name, data)))
}

pub fn package(manifest: &Manifest, extra: &[(&str, &[u8])]) -> Vec<u8> {
    let json = manifest.to_json().unwrap();
    let base: [(&str, &[u8]); 2] = [(MANIFEST, json.as_bytes()), (&manifest.entry, b"entry")];
    archive(&[&base[..], extra].concat())
}

pub fn patch(mut bytes: Vec<u8>, from: &[u8], to: &[u8]) -> Vec<u8> {
    let hits: Vec<usize> =
        bytes.windows(from.len()).enumerate().filter(|(_, w)| *w == from).map(|(i, _)| i).collect();
    hits.into_iter().for_each(|i| bytes[i..i + from.len()].copy_from_slice(to));
    bytes
}
