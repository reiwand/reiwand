use super::form::{FIELDS, PASS_KEYS, keyed, list};
use anyhow::{Result, anyhow};
use reiwand_core::message::said;
use serde_json::{Map, Value, json};

pub(crate) fn lighter(json: &mut Value, set: &Map<String, Value>, remove: bool) -> Result<bool> {
    let id = json["id"].as_str().unwrap_or_default().to_owned();
    if remove {
        json["lighter"] = Value::Null;
        return Ok(false);
    }
    let made = !json["lighter"].is_object();
    if made {
        let cost = json!({ "taps": 1, "passes": 1, "scale": 1 });
        json["lighter"] =
            json!({ "passes": [super::whole::pass(format!("fs_{id}_lite"))], "cost": cost, "up_to": null });
    }
    for (k, v) in set {
        keyed(k, &["cost", "up_to"])?;
        match v {
            Value::Null => json["lighter"][k] = Value::Null,
            v => json["lighter"][k] = v.clone(),
        }
    }
    Ok(made)
}

pub(crate) struct PassChange<'a> {
    pub index: Option<usize>,
    pub set: &'a Map<String, Value>,
    pub clear: &'a [String],
    pub remove: bool,
    pub lighter: bool,
}

pub(crate) fn pass(json: &mut Value, c: &PassChange) -> Result<()> {
    let PassChange { index, set, clear, remove, lighter } = *c;
    let id = json["id"].as_str().unwrap_or_default().to_owned();
    if lighter && !json["lighter"].is_object() {
        self::lighter(json, &Map::new(), false)?;
    }
    let (passes, entry) = match lighter {
        true => (list(&mut json["lighter"], "passes"), format!("fs_{id}_lite_")),
        false => (list(json, "passes"), format!("fs_{id}_")),
    };
    let missing = || anyhow!(said("editor.no_pass", "there is no pass %1", &[&index.unwrap_or_default()]));
    let at = match index {
        Some(i) if i < passes.len() => i,
        Some(_) => return Err(missing()),
        None if remove => return Err(missing()),
        None => {
            passes.push(super::whole::pass(format!("{entry}{}", passes.len())));
            passes.len() - 1
        }
    };
    if remove {
        passes.remove(at);
        return Ok(());
    }
    for (k, v) in set {
        keyed(k, &PASS_KEYS)?;
        passes[at][k] = v.clone();
    }
    for k in clear {
        keyed(k, &PASS_KEYS[1..])?;
        passes[at][k] = super::whole::cleared_pass(k);
    }
    Ok(())
}

pub(crate) fn field(json: &mut Value, name: &str, value: &Value) -> Result<()> {
    keyed(name, &FIELDS)?;
    match value {
        Value::Null => json[name] = super::whole::cleared_field(name),
        v => json[name] = v.clone(),
    }
    Ok(())
}
