use crate::Error;
use crate::said;
use crate::side::Phone;
use crate::state::{api, linked};
use reiwand_sync::client::{Fault, rotation};
use serde_json::{Value, json};

pub(crate) async fn device_code(code: &str, action: &str, mut body: Value) -> Result<Value, Error> {
    let letters: String = code.chars().filter(char::is_ascii_alphanumeric).collect();
    if letters.len() != 8 {
        return Err(said("android.give_letters_shown", "give the 8 letters shown on the new device", &[]));
    }
    linked()?;
    body["user_code"] = json!(letters.to_uppercase());
    let (api, _) = api()?;
    Ok(api.post(&format!("/api/v1/auth/device/{action}"), body).await?)
}

pub(crate) async fn rotate_now() -> Result<Value, Error> {
    crate::level::syncing()?;
    rotation::check(&Phone)?;
    let (api, mut secrets) = api()?;
    rotation::rotate(&Phone, &api, &mut secrets, false).await?.ok_or_else(|| Fault::RotatedElsewhere.into())
}
