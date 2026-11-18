use crate::Outcome;
use crate::fill::{Format, complete};
use anyhow::{Context, bail};
use reiwand_carry::{Content, Cost, FORMAT, Manifest, is_sealed, seal, unseal};
use reiwand_core::explicit::read;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Cursor, Read, Seek, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

const CONTENT: &str = "files/content.json";
const OWN: &str = "files/own/";
const HEAD: &str = "carry.json";

fn text(archive: &mut ZipArchive<impl Read + Seek>, name: &str) -> anyhow::Result<Vec<u8>> {
    let mut out = Vec::new();
    archive.by_name(name).with_context(|| format!("{name} is missing"))?.read_to_end(&mut out)?;
    Ok(out)
}

fn free_values(doc: &mut Value) {
    let each = |key: &str, field: &str, value: Value, doc: &mut Value| {
        for entry in doc[key].as_array_mut().into_iter().flatten().filter_map(Value::as_object_mut) {
            entry.entry(field).or_insert_with(|| value.clone());
        }
    };
    each("lists", "rules", Value::Null, doc);
    each("providers", "config", serde_json::json!({}), doc);
}

fn converted<R: Read + Seek, W: Write + Seek>(
    archive: &mut ZipArchive<R>,
    out: W,
) -> anyhow::Result<Option<()>> {
    let mut head: Value = serde_json::from_slice(&text(archive, HEAD)?)?;
    match head["format"].as_u64() {
        Some(found) if found == u64::from(FORMAT) => return Ok(None),
        Some(1) => {}
        other => bail!("the file has format {other:?}; only format 1 can be converted"),
    }
    let mut doc: Value = serde_json::from_slice(&text(archive, CONTENT)?)?;
    complete(Format::Carry, &mut doc);
    free_values(&mut doc);
    let content: Content = read(CONTENT, &doc)?;
    head["format"] = FORMAT.into();
    let mut manifest: Manifest = read(HEAD, &head)?;
    manifest.sections = content.sections();
    let json = serde_json::to_vec(&content)?;
    manifest.sums.insert(CONTENT.to_owned(), reiwand_hex(&Sha256::digest(&json)));
    let mut zip = ZipWriter::new(out);
    let packed = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    zip.start_file(CONTENT, packed)?;
    zip.write_all(&json)?;
    for index in 0..archive.len() {
        let entry = archive.by_index_raw(index)?;
        let name = entry.name().unwrap_or_default().into_owned();
        if name.starts_with(OWN) {
            zip.raw_copy_file(entry)?;
        } else if name != CONTENT && name != HEAD {
            bail!("the file holds an entry this program does not know: {name}");
        }
    }
    zip.start_file(HEAD, packed)?;
    zip.write_all(&serde_json::to_vec(&manifest)?)?;
    zip.finish()?;
    Ok(Some(()))
}

fn reiwand_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn file(path: &Path, password: Option<&str>, check: bool) -> anyhow::Result<Outcome> {
    let mut magic = [0u8; 8];
    let mut source = File::open(path).with_context(|| format!("cannot read {}", path.display()))?;
    let sealed = source.read_exact(&mut magic).is_ok() && is_sealed(&magic);
    source.rewind()?;
    let staged = path.with_extension("reiwand.upgrading");
    let done = if sealed {
        let password = password.context("the file is encrypted: give its password (--password-file FILE)")?;
        let mut data = Vec::new();
        source.read_to_end(&mut data)?;
        let mut archive = ZipArchive::new(Cursor::new(unseal(&data, password)?.to_vec()))?;
        let mut out = Cursor::new(Vec::new());
        let done = converted(&mut archive, &mut out)?;
        if done.is_some() && !check {
            fs::write(&staged, seal(&out.into_inner(), password, Cost::default())?)?;
        }
        done
    } else {
        let mut archive = ZipArchive::new(std::io::BufReader::new(source))?;
        if check {
            converted(&mut archive, Cursor::new(Vec::new()))?
        } else {
            let mut out = std::io::BufWriter::new(File::create(&staged)?);
            let done = converted(&mut archive, &mut out)?;
            out.flush()?;
            done
        }
    };
    let Some(()) = done else {
        let _ = fs::remove_file(&staged);
        return Ok(Outcome::Nothing);
    };
    if !check {
        if let Err(e) = reiwand_carry::open(&staged, password, reiwand_carry::Limits::default()) {
            let _ = fs::remove_file(&staged);
            bail!("the converted file does not read back, the original is untouched: {e}");
        }
        fs::rename(&staged, path)?;
    }
    Ok(Outcome::Upgraded { from: 1, to: i64::from(FORMAT) })
}

pub fn sealed(path: &Path) -> bool {
    let mut magic = [0u8; 8];
    File::open(path).is_ok_and(|mut f| f.read_exact(&mut magic).is_ok() && is_sealed(&magic))
}
