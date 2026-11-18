use crate::script::tell;
use crate::state::live;
use serde_json::json;

pub fn update(folder: reiwand_web::Folder) {
    let old = live(|l| {
        l.files.extend(folder.files.iter().map(|(t, p)| (t.clone(), p.clone())));
        l.folders.insert(folder.prop.clone(), folder.clone())
    });
    let old = old.map(|f| f.files).unwrap_or_default();
    let before: Vec<String> = old.iter().map(|(t, _)| t.clone()).collect();
    let now: Vec<String> = folder.files.iter().map(|(t, _)| t.clone()).collect();
    live(|l| before.iter().filter(|t| !now.contains(t)).for_each(|t| drop(l.files.remove(t))));
    let paths: std::collections::HashMap<&str, &std::path::Path> =
        old.iter().chain(&folder.files).map(|(t, p)| (t.as_str(), p.as_path())).collect();
    let path = |t: &str| paths.get(t).map(|p| p.to_path_buf());
    if let Some(notice) =
        reiwand_format::page::files(&folder.prop, &before, &now, path).filter(|_| folder.all)
    {
        tell(&json!({ "files": notice }));
    }
}
