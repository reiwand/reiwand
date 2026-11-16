use anyhow::{Result, ensure};
use reiwand_format::{Kind, Limits, Manifest, Text, build, keygen};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

fn package(dir: &Path) -> Result<String> {
    let src = dir.join("src");
    fs::create_dir_all(&src)?;
    let manifest = Manifest {
        schema: reiwand_format::SCHEMA.into(),
        content: Some(reiwand_format::Content {
            rating: reiwand_format::Rating::General,
            descriptors: Vec::new(),
        }),
        photosensitive: Some(false),
        origin: Some(reiwand_format::Origin::default()),
        id: "org.example.check".into(),
        version: "1.0.0".into(),
        title: Text::Plain("Check".into()),
        description: "A package of the app check".into(),
        license: "CC0-1.0".into(),
        kind: Kind::Image,
        entry: "image.png".into(),
        ..Default::default()
    };
    fs::write(src.join("manifest.json"), manifest.to_json()?)?;
    image::RgbaImage::from_pixel(4, 4, image::Rgba([40, 80, 120, 255])).save(src.join("image.png"))?;
    let out = dir.join("check.wpak");
    build(&src, &out, &Limits::default())?;
    Ok(out.to_string_lossy().into_owned())
}

pub fn sample(name: &str) -> Result<Value> {
    ensure!(crate::bus::checking(), "sample repositories are made only by reiwand-app --check");
    ensure!(!name.is_empty() && !name.contains(['/', '.']), "a sample needs a plain folder name");
    let dir = crate::scenefile::projects_dir().join(name);
    fs::create_dir_all(&dir)?;
    let (root, online) = (keygen(None)?, keygen(Some("check".into()))?);
    let file = |name: &str, text: &str| -> Result<String> {
        let path = dir.join(name);
        fs::write(&path, text)?;
        Ok(path.to_string_lossy().into_owned())
    };
    Ok(json!({
        "repo": dir.join("repo").to_string_lossy(),
        "root_pub": file("root.pub", &root.public)?, "root_key": file("root.key", &root.secret)?,
        "online_pub": file("online.pub", &online.public)?, "online_key": file("online.key", &online.secret)?,
        "wpak": package(&dir)?,
    }))
}
