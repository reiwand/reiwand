use crate::model::{Content, Manifest, OwnFile};
use crate::safe::{check_id, hex};
use crate::seal::{Cost, seal};
use crate::{Error, FORMAT, Result};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

#[derive(Debug)]
pub struct Export {
    pub program: String,
    pub version: String,
    pub created: String,
    pub content: Content,
    pub own: Vec<(OwnFile, PathBuf)>,
    pub password: Option<String>,
    pub cost: Cost,
}

fn put<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    name: &str,
    deflate: bool,
    mut from: impl Read,
) -> Result<String> {
    let method = if deflate { CompressionMethod::Deflated } else { CompressionMethod::Stored };
    zip.start_file(name, SimpleFileOptions::default().compression_method(method).large_file(true))?;
    let (mut sum, mut buf) = (Sha256::new(), vec![0u8; 1 << 16]);
    loop {
        let n = from.read(&mut buf)?;
        if n == 0 {
            break;
        }
        sum.update(&buf[..n]);
        zip.write_all(&buf[..n])?;
    }
    Ok(hex(&sum.finalize()))
}

fn build<W: Write + Seek>(out: W, export: &Export) -> Result<()> {
    let mut content = export.content.clone();
    content.own = export.own.iter().map(|(own, _)| own.clone()).collect();
    if content.secrets.is_some() && export.password.is_none() {
        return Err(Error::Invalid("secrets are only carried in an encrypted file".into()));
    }
    for own in &content.own {
        check_id(&own.id)?;
    }
    let mut zip = ZipWriter::new(out);
    let mut sums = BTreeMap::new();
    let json = serde_json::to_vec(&content)?;
    sums.insert(
        "files/content.json".to_string(),
        put(&mut zip, "files/content.json", true, json.as_slice())?,
    );
    for (own, path) in &export.own {
        let name = format!("files/own/{}", own.id);
        sums.insert(name.clone(), put(&mut zip, &name, false, File::open(path)?)?);
    }
    let manifest = Manifest {
        format: FORMAT,
        program: export.program.clone(),
        version: export.version.clone(),
        created: export.created.clone(),
        sections: content.sections(),
        encrypted: export.password.is_some(),
        sums,
    };
    put(&mut zip, "carry.json", true, serde_json::to_vec(&manifest)?.as_slice())?;
    zip.finish()?;
    Ok(())
}

pub fn write_bytes(export: &Export) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    build(&mut cursor, export)?;
    match &export.password {
        Some(password) => seal(&cursor.into_inner(), password, export.cost),
        None => Ok(cursor.into_inner()),
    }
}

pub fn write(path: &Path, export: &Export) -> Result<()> {
    if export.password.is_some() {
        return Ok(std::fs::write(path, write_bytes(export)?)?);
    }
    let mut out = std::io::BufWriter::new(File::create(path)?);
    build(&mut out, export)?;
    out.flush()?;
    Ok(())
}
