use crate::scene::qobject::SceneEditor;
use crate::scenefile::Shown;
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::path::Path;

fn answer(result: anyhow::Result<Value>) -> QString {
    QString::from(&result.unwrap_or_else(|e| json!({ "error": e.shown() })).to_string())
}

impl SceneEditor {
    pub fn copy_facts(&self, package: &QString) -> QString {
        answer(reiwand_editor::copying::facts(Path::new(&package.to_string())))
    }

    pub fn examples(&self) -> QString {
        QString::from(&Value::from(crate::builtin::list()).to_string())
    }

    pub fn example_package(&self, name: &QString) -> QString {
        let made = crate::builtin::package(&name.to_string(), &crate::builtin::room());
        answer(made.map(|path| json!({ "path": path })))
    }

    pub fn copy_lineage(&self, package: &QString) -> QString {
        answer(reiwand_editor::copying::lineage(Path::new(&package.to_string())))
    }

    pub fn copy_into(&self, package: &QString, folder: &QString, id: &QString, mine: bool) -> QString {
        let id = Some(id.to_string()).filter(|i| !i.is_empty());
        let made = reiwand_editor::copying::extract(
            Path::new(&package.to_string()),
            Path::new(&folder.to_string()),
            id.as_deref(),
            mine,
        );
        answer(made.map(|id| json!({ "id": id, "folder": folder.to_string() })))
    }
}
