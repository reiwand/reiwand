use crate::told::{fill, told};
use crate::usage;
use anyhow::Result;
use std::path::Path;

pub fn repository(text: &str) -> Result<String> {
    if text.contains("://") {
        return Ok(text.to_owned());
    }
    if !Path::new(text).is_dir() {
        return Err(usage(fill(told("%1 is neither a folder nor a URL"), &[&text])));
    }
    let absolute = std::path::absolute(text)?;
    Ok(format!("file://{}/", absolute.display().to_string().trim_end_matches('/')))
}

#[cfg(test)]
mod tests {
    use super::repository;

    #[test]
    fn a_folder_becomes_a_plain_file_url() {
        let tmp = tempfile::tempdir().unwrap();
        let folder = tmp.path().display().to_string();
        assert_eq!(repository(&folder).unwrap(), format!("file://{folder}/"));
        assert_eq!(repository(&format!("{folder}/")).unwrap(), format!("file://{folder}/"));
    }

    #[test]
    fn urls_pass_through_and_missing_folders_are_refused() {
        let url = "https://example.org/repo";
        assert_eq!(repository(url).unwrap(), url);
        let error = repository("no/such/folder").unwrap_err().to_string();
        assert!(error.contains("neither a folder nor a URL"), "{error}");
    }
}
