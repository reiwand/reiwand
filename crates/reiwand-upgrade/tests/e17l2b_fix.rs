use reiwand_upgrade::fill::{Format, complete};
use serde_json::Value;

#[test]
#[ignore]
fn fix_listed_files() {
    let list = std::fs::read_to_string(std::env::var("FIX_LIST").unwrap()).unwrap();
    for path in list.lines().filter(|l| !l.is_empty()) {
        let text = std::fs::read_to_string(path).unwrap();
        let original: Value = serde_json::from_str(&text).unwrap();
        let mut doc = original.clone();
        complete(Format::Scene, &mut doc);
        let mut out = original.clone();
        out["particles"] = doc["particles"].clone();
        if let Some(layers) = out["layers"].as_array_mut() {
            for (i, layer) in layers.iter_mut().enumerate() {
                if layer["type"] == "particles" {
                    *layer = doc["layers"][i].clone();
                }
            }
        }
        let pretty = text.contains("\n  ");
        let body = if pretty {
            serde_json::to_string_pretty(&out).unwrap() + "\n"
        } else {
            serde_json::to_string(&out).unwrap()
        };
        std::fs::write(path, body).unwrap();
    }
}
