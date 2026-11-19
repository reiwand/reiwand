use crate::state::{all, engine, live, send};
use reiwand_web::FromWeb;
use serde_json::{Value, json};

fn post(nonce: &str) -> String {
    format!(
        "(() => {{ const log = console.debug.bind(console); return (m) => log(\"{}\" + JSON.stringify(m)); }})()",
        prefix(nonce)
    )
}

pub fn prefix(nonce: &str) -> String {
    format!("reiwand:{nonce}:")
}

fn sources(view: u32) -> Vec<(String, bool)> {
    let Some(engine) = engine() else { return Vec::new() };
    let init = live(|l| {
        let s = &l.settings;
        json!({
            "properties": l.views.get(&view).unwrap_or(&l.properties),
            "kinds": engine.start.shape.get("kinds").cloned().unwrap_or(json!({})),
            "labels": engine.start.shape.get("labels").cloned().unwrap_or(json!({})),
            "settings": {"fps": s.fps, "quality": s.quality, "locale": s.locale, "volume": s.volume},
            "lights": l.lights.contains(&view),
        })
    });
    let mut scripts = vec![(reiwand_format::page::api(&init, &post(&engine.nonce)), false)];
    if engine.start.mask_gpu {
        scripts.push((format!("({})(globalThis);", reiwand_format::page::MASK.trim_end()), true));
    }
    scripts.push((format!("({})(globalThis);", reiwand_format::page::WEBRTC.trim_end()), true));
    scripts.push((reiwand_format::page::VOLUME.into(), true));
    scripts.extend(engine.preload.clone().map(|p| (p, false)));
    scripts
}

pub fn install(view: u32) {
    let list: Vec<Value> =
        sources(view).into_iter().map(|(s, f)| json!({"source": s, "frames": f})).collect();
    crate::qt::scripts(view, &Value::Array(list).to_string());
}

pub fn tell(message: &Value) {
    all().iter().for_each(|v| tell_view(v.id, message));
}

pub fn tell_view(id: u32, message: &Value) {
    if crate::state::view(id).is_some_and(|v| v.loaded.get()) {
        crate::qt::eval(id, &format!("window.__reiwand && window.__reiwand({message})"));
    }
}

fn reply(view: u32, id: u64, value: Value) {
    direct(view, &json!({"reply": {"id": id, "value": value}}));
}

fn direct(view: u32, message: &Value) {
    crate::qt::eval(view, &format!("window.__reiwand({message})"));
}

fn tokens(prop: &str, every: bool) -> Option<Vec<String>> {
    live(|l| {
        let folder = l.folders.get(prop)?;
        (every == folder.all || !every).then(|| folder.files.iter().map(|(t, _)| t.clone()).collect())
    })
}

fn random(list: &[String]) -> Value {
    let seed =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.subsec_nanos());
    match list.len() {
        0 => Value::Null,
        n => Value::String(list[seed as usize % n].clone()),
    }
}

pub fn message(view: u32, text: &str) {
    let Ok(Value::Object(message)) = serde_json::from_str::<Value>(text) else { return };
    let id = message.get("id").and_then(Value::as_u64).unwrap_or(0);
    if message.get("audio").is_some() && !live(|l| std::mem::replace(&mut l.audio, true)) {
        send(&FromWeb::Wants { audio: true });
    }
    if message.get("media").is_some() {
        let last = live(|l| {
            l.media = true;
            l.heard.clone()
        });
        let last = last.unwrap_or_else(|| json!({ "enabled": false }));
        direct(view, &json!({ "media": last }));
    }
    if let Some(prop) = message.get("files").and_then(Value::as_str) {
        reply(view, id, tokens(prop, true).map_or(Value::Null, |t| json!(t)));
    }
    if let Some(prop) = message.get("random").and_then(Value::as_str) {
        reply(view, id, random(&tokens(prop, false).unwrap_or_default()));
    }
    if let Some(rgb) = message.get("rgb") {
        lit(view, rgb);
    }
    if let Some(prop) = message.get("open").and_then(Value::as_str) {
        send(&FromWeb::Open { view, prop: prop.chars().take(64).collect() });
    }
    if let Some(error) = message.get("error").and_then(Value::as_str)
        && live(|l| l.said.allow())
    {
        send(&FromWeb::Console { view, line: error.chars().take(512).collect() });
    }
}

fn lit(view: u32, rgb: &Value) {
    if !live(|l| l.lights.contains(&view)) {
        return;
    }
    static LAST: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);
    let side = |key: &str, most: u64| rgb[key].as_u64().filter(|n| (1..=most).contains(n)).map(|n| n as u32);
    let (Some(w), Some(h)) = (side("width", 64), side("height", 16)) else { return };
    let byte = |v: &Value| v.as_u64().filter(|v| *v < 256).map(|v| v as u8);
    let data: Option<Vec<u8>> = rgb["data"].as_array().and_then(|list| list.iter().map(byte).collect());
    let Some(data) = data.filter(|d| d.len() == (w * h * 3) as usize) else { return };
    let mut last = LAST.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if last.is_some_and(|at| at.elapsed() < std::time::Duration::from_millis(33)) {
        return;
    }
    *last = Some(std::time::Instant::now());
    send(&FromWeb::Rgb { view, size: (w, h), rgb: data });
}

pub fn heard(heard: Value) {
    let asked = live(|l| {
        let mut kept = heard.clone();
        let before = l.heard.as_ref().and_then(|h| h.get("cover")).filter(|_| heard.get("cover").is_none());
        if let Some(cover) = before {
            kept["cover"] = cover.clone();
        }
        l.heard = Some(kept);
        l.media
    });
    if asked {
        tell(&json!({ "media": heard }));
    }
}

pub fn replay(view: u32) {
    for message in live(|l| crate::replay::missed(l, view)) {
        tell_view(view, &message);
    }
}
