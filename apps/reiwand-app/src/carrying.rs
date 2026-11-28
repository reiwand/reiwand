use crate::args::{Out, fail, fixture, invalid};
use crate::backend::qobject::Backend;
use crate::bus::{self, Daemon};
use cxx_qt_lib::QString;
use serde_json::{Map, Value};
use std::path::PathBuf;
use std::pin::Pin;
use zeroize::Zeroizing;

pub fn own_bytes(dir: Option<PathBuf>) -> u64 {
    fn sum(dir: &std::path::Path, depth: u8) -> u64 {
        let Ok(entries) = std::fs::read_dir(dir) else { return 0 };
        entries
            .flatten()
            .map(|e| match e.metadata() {
                Ok(m) if m.is_file() => m.len(),
                Ok(m) if m.is_dir() && depth > 0 => sum(&e.path(), depth - 1),
                _ => 0,
            })
            .sum()
    }
    dir.map_or(0, |d| sum(&d.join("reiwand").join("files"), 3))
}

pub fn export_options(options: &str, password: &str) -> Result<String, reiwand_client::Error> {
    let mut map: Map<String, Value> = serde_json::from_str(options).map_err(|e| invalid(e.to_string()))?;
    map.insert("password".into(), Value::String(password.to_owned()));
    Ok(Value::Object(map).to_string())
}

impl Backend {
    pub fn carry(
        self: Pin<&mut Self>,
        tag: &QString,
        member: &QString,
        path: &QString,
        password: &QString,
        json: &QString,
    ) {
        let (member, path, json) = (member.to_string(), path.to_string(), json.to_string());
        let password = Zeroizing::new(password.to_string());
        self.run(tag, true, async move {
            let daemon = bus::daemon().await?;
            let Daemon::Bus(c) = daemon else {
                return match member.as_str() {
                    "Export" | "PreviewImport" | "ImportChosen" => Ok(fixture("Carry1", &member)),
                    _ => Err(invalid(format!("{member} is not a carry call"))),
                };
            };
            match member.as_str() {
                "Export" => {
                    let options = Zeroizing::new(export_options(&json, &password)?);
                    c.carry.export(&path, &options).await.map(|r| r.out().to_string()).map_err(fail)
                }
                "PreviewImport" => c.carry.preview_import(&path, &password).await.map_err(fail),
                "ImportChosen" => c
                    .carry
                    .import_chosen(&path, &password, &json)
                    .await
                    .map(|r| r.out().to_string())
                    .map_err(fail),
                _ => Err(invalid(format!("{member} is not a carry call"))),
            }
        });
    }

    pub fn own_size(&self) -> f64 {
        own_bytes(dirs::data_dir()) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::{export_options, own_bytes};

    #[test]
    fn the_password_joins_the_options_and_nothing_else() {
        let text = export_options("{\"own\":true,\"secrets\":true}", "correct horse").unwrap();
        let value: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(value["password"], "correct horse");
        assert_eq!(value["own"], true);
        assert!(export_options("[1]", "x").is_err());
    }

    #[test]
    fn the_own_size_adds_the_files_of_the_library_folder() {
        let room = tempfile::tempdir().unwrap();
        let base = room.path().to_path_buf();
        let files = base.join("reiwand").join("files");
        std::fs::create_dir_all(files.join("nested")).unwrap();
        std::fs::write(files.join("a.bin"), [0u8; 100]).unwrap();
        std::fs::write(files.join("nested").join("b.bin"), [0u8; 28]).unwrap();
        assert_eq!(own_bytes(Some(base.clone())), 128);
        assert_eq!(own_bytes(Some(base.join("missing"))), 0);
        assert_eq!(own_bytes(None), 0);
    }
}
