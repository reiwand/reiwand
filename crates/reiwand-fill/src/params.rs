use super::{Customs, Report, effects};
use serde_json::{Map, Value};

pub(super) fn complete(map: &mut Map<String, Value>, customs: &Customs, report: &mut Report) {
    let Some(effect) = map.get("effect").and_then(Value::as_str).map(str::to_owned) else { return };
    let Some(wanted) = customs.get(&effect).or_else(|| effects().get(&effect)) else { return };
    let params = map.entry("params").or_insert_with(|| Value::Object(Map::new()));
    let Some(params) = params.as_object_mut() else { return };
    for (id, value) in wanted {
        if !params.contains_key(id) {
            params.insert(id.clone(), value.clone());
            report.note("EffectUse.params", id);
        }
    }
}
