mod common;
use serde_json::{Value, json};
use std::path::PathBuf;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frozen")
}

fn effects() -> Value {
    let mut out = serde_json::Map::new();
    for d in reiwand_scene::builtin() {
        let params: Vec<Value> =
            d.params.iter().map(|p| json!([p.id, serde_json::to_value(&p.prefill).unwrap()])).collect();
        out.insert(d.id.clone(), Value::Array(params));
    }
    Value::Object(out)
}

fn shortest(value: &mut Value) {
    match value {
        Value::Number(n) => {
            if let Some(x) = n.as_f64().filter(|x| n.is_f64() && x.is_finite() && (*x as f32) as f64 == *x) {
                let text = (x as f32).to_string();
                if let Some(short) = text.parse::<f64>().ok().and_then(serde_json::Number::from_f64) {
                    *n = short;
                }
            }
        }
        Value::Array(list) => list.iter_mut().for_each(shortest),
        Value::Object(map) => map.values_mut().for_each(shortest),
        _ => {}
    }
}

fn live() -> Vec<(&'static str, Value)> {
    let mut all = common::live_schemas();
    all.push(("effects", effects()));
    all
}

#[test]
#[ignore = "regenerates the frozen schemas from the code of today"]
fn freeze() {
    for (name, mut value) in live() {
        shortest(&mut value);
        let text = serde_json::to_string(&value).unwrap();
        std::fs::write(dir().join(format!("{name}.json")), text + "\n").unwrap();
    }
}
