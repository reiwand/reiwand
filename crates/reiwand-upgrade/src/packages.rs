use crate::sink::Sink;
use anyhow::Context;
use serde_json::Value;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use zip::ZipArchive;

pub struct Converted {
    pub bytes: Vec<u8>,
    pub was_signed: bool,
}

pub fn converted(path: &Path) -> anyhow::Result<Option<Converted>> {
    let work = tempfile::tempdir()?;
    let tree = work.path().join("package");
    ZipArchive::new(BufReader::new(File::open(path)?))
        .and_then(|mut zip| zip.extract(&tree))
        .with_context(|| format!("cannot unpack {}", path.display()))?;
    let mut inner = Sink::plain();
    let changed = crate::scenes::walk_in(&tree, &mut inner)?;
    if let Some((file, reason)) = inner.skipped().first() {
        anyhow::bail!("{} cannot be converted: {reason}", file.display());
    }
    if changed == 0 {
        return Ok(None);
    }
    let signature = tree.join(reiwand_format::SIGNATURE);
    let was_signed = signature.is_file();
    if was_signed {
        fs::remove_file(&signature)?;
    }
    let out = work.path().join("package.wpak");
    reiwand_format::build(&tree, &out, &reiwand_format::Limits::default())
        .with_context(|| format!("cannot pack {} again", path.display()))?;
    Ok(Some(Converted { bytes: fs::read(out)?, was_signed }))
}

pub fn wpak(path: &Path, sink: &mut Sink) -> anyhow::Result<bool> {
    let Some(done) = converted(path)? else { return Ok(false) };
    sink.save(path, &done.bytes)?;
    if done.was_signed {
        sink.lost_signature(path.display().to_string());
    }
    Ok(true)
}

fn records(doc: &mut Value) -> Option<&mut Vec<Value>> {
    if doc.is_array() {
        return doc.as_array_mut();
    }
    doc.get_mut("content")?.as_array_mut()
}

pub fn installed(data: &Path, sink: &mut Sink) -> anyhow::Result<usize> {
    let file = data.join("installed.json");
    let Ok(text) = fs::read_to_string(&file) else { return Ok(0) };
    let mut doc: Value = serde_json::from_str(&text).context("installed.json is not valid JSON")?;
    let mut done = 0;
    let count = records(&mut doc).map_or(0, |list| list.len());
    for at in 0..count {
        let Some(record) = records(&mut doc).and_then(|list| list.get_mut(at)) else { break };
        let Some(path) = record.get("path").and_then(Value::as_str).map(std::path::PathBuf::from) else {
            continue;
        };
        if !path.is_file() {
            continue;
        }
        let converted = match converted(&path) {
            Ok(Some(package)) => package,
            Ok(None) => continue,
            Err(e) => {
                sink.skip(&path, format!("{e:#}"));
                continue;
            }
        };
        let bytes = converted.bytes;
        sink.save(&path, &bytes)?;
        if converted.was_signed || record.get("signer").is_some_and(|signer| !signer.is_null()) {
            record["signer"] = Value::Null;
            let name = |key| record.get(key).and_then(Value::as_str).unwrap_or("").to_owned();
            sink.lost_signature(format!("{} {}", name("id"), name("version")));
        }
        record["sha256"] = Value::from(reiwand_format::sha256(&bytes[..])?);
        record["size"] = Value::from(bytes.len() as u64);
        done += 1;
        sink.save(&file, (serde_json::to_string_pretty(&doc)? + "\n").as_bytes())?;
    }
    Ok(done)
}
