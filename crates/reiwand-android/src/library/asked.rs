use crate::Error;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
enum Asked {
    Props(crate::props::Request),
    Graded(crate::graded::Request),
    Blanked(crate::blanked::Request),
    Converted(crate::converted::Request),
    Kin(crate::kin::Request),
    Labels(super::labels::Request),
    Library(super::Request),
}

pub(crate) fn request(text: &str, on_battery: bool) -> String {
    let answer = match serde_json::from_str(text) {
        Ok(Asked::Props(r)) => crate::props::request(r),
        Ok(Asked::Graded(r)) => crate::graded::request(r),
        Ok(Asked::Blanked(r)) => crate::blanked::request(r),
        Ok(Asked::Converted(r)) => crate::converted::request(r),
        Ok(Asked::Kin(r)) => crate::kin::request(r),
        Ok(Asked::Labels(r)) => super::labels::request(r),
        Ok(Asked::Library(r)) => super::with(|lib| super::answer(lib, r, on_battery)),
        Err(_) => serde_json::from_str::<super::Request>(text).map(|_| Value::Null).map_err(Error::from),
    };
    answer.unwrap_or_else(|e| e.reply()).to_string()
}
