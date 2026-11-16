#![allow(dead_code)]

use reiwand_format::{Kind, Limits, read};
use reiwand_scene::{Quality, Runtime, Values};
use std::io::{Cursor, Write};
use std::sync::Arc;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn zipped(files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (index, (name, data)) in files.iter().enumerate() {
        let method = if index % 2 == 0 { CompressionMethod::Deflated } else { CompressionMethod::Stored };
        zip.start_file(name.as_str(), SimpleFileOptions::default().compression_method(method)).unwrap();
        zip.write_all(data).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

pub fn opened(bytes: Vec<u8>) -> Result<usize, String> {
    let mut package = read(Cursor::new(bytes), &Limits::default()).map_err(|e| e.to_string())?;
    let paths: Vec<String> = package.entries.iter().map(|e| e.path.clone()).collect();
    let read = paths.iter().filter_map(|p| package.read_file(p).ok()).map(|d| d.len()).sum();
    if matches!(package.manifest.kind, Kind::Scene | Kind::Assets) {
        let scene = package.scene().map_err(|e| e.to_string())?;
        package.luts(&scene).map_err(|e| e.to_string())?;
        played(scene);
    }
    Ok(read)
}

pub fn played(scene: reiwand_scene::Scene) {
    let mut runtime = Runtime::new(Arc::new(scene), Values::new());
    for step in [0.0, 1.0 / 60.0, 0.5, 30.0] {
        runtime.advance(step);
        for quality in [Quality::Q0, Quality::Q3] {
            drop(runtime.frame([0.0, 0.0, 1920.0, 1080.0], quality));
        }
    }
}

pub fn sizes(bytes: &[u8], value: u32) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for (signature, offsets) in [(b"PK\x01\x02", [20usize, 24]), (b"PK\x03\x04", [18, 22])] {
        let starts = bytes.windows(4).enumerate().filter(|(_, w)| w == signature).map(|(i, _)| i);
        for start in starts.collect::<Vec<_>>() {
            for offset in offsets {
                let at = start + offset;
                let mut copy = bytes.to_vec();
                if at + 4 <= copy.len() {
                    copy[at..at + 4].copy_from_slice(&value.to_le_bytes());
                    out.push(copy);
                }
            }
        }
    }
    out
}
