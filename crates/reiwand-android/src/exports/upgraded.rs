use super::on;
use jni::objects::{JObject, JString};
use jni::sys::jboolean;
use jni::{EnvUnowned, jni_mangle};

#[jni_mangle("io.github.reiwand.Native")]
pub fn upgrade<'a>(mut env: EnvUnowned<'a>, _: JObject<'a>, files: JString<'a>) -> jboolean {
    on(&mut env, |env| {
        let files = files.try_to_string(env)?;
        reiwand_upgrade::client::run(std::path::Path::new(&files))?;
        Ok(true)
    })
}
