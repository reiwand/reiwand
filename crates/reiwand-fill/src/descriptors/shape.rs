use serde_json::{Map, Value, json};

fn fill(object: &mut Map<String, Value>, fields: &[(&str, Value)]) -> bool {
    let mut changed = false;
    for (key, value) in fields {
        if !object.contains_key(*key) {
            object.insert((*key).to_owned(), value.clone());
            changed = true;
        }
    }
    changed
}

fn each(value: Option<&mut Value>, mut step: impl FnMut(&mut Map<String, Value>) -> bool) -> bool {
    let items = value.and_then(Value::as_array_mut).into_iter().flatten();
    items.filter_map(Value::as_object_mut).fold(false, |done, item| step(item) | done)
}

fn cost(value: Option<&mut Value>) -> bool {
    let Some(cost) = value.and_then(Value::as_object_mut) else { return false };
    fill(cost, &[("taps", json!(1)), ("passes", json!(1)), ("scale", json!(1))])
}

fn pass(pass: &mut Map<String, Value>) -> bool {
    let none = Value::Null;
    fill(
        pass,
        &[
            ("entry", json!("")),
            ("scale", json!(1)),
            ("inputs", json!(["previous"])),
            ("output", json!("next")),
            ("target", none.clone()),
            ("copy", none.clone()),
            ("blend", none.clone()),
            ("vertex", none.clone()),
            ("grid", none.clone()),
            ("swap", none),
        ],
    )
}

fn param(param: &mut Map<String, Value>) -> bool {
    let none = Value::Null;
    let ranged = matches!(param.get("type").and_then(Value::as_str), Some("float" | "int" | "vec2"));
    let (low, high) = if ranged { (json!(f32::MIN), json!(f32::MAX)) } else { (none.clone(), none.clone()) };
    let mut changed = fill(
        param,
        &[
            ("min", low.clone()),
            ("max", high.clone()),
            ("uses", none.clone()),
            ("unit", none.clone()),
            ("tools", json!([])),
            ("label", json!({})),
            ("help", json!({})),
            ("values", json!([])),
            ("channels", none.clone()),
            ("paint", none.clone()),
            ("yields_to", none.clone()),
            ("waits_for", none.clone()),
            ("tool", none),
        ],
    );
    for (key, bound) in [("min", low), ("max", high)] {
        if ranged && param.get(key).is_some_and(Value::is_null) {
            param.insert(key.to_owned(), bound);
            changed = true;
        }
    }
    changed |= each(param.get_mut("values"), |entry| fill(entry, &[("label", json!({}))]));
    changed
}

fn tune(tune: &mut Map<String, Value>) -> bool {
    fill(tune, &[("params", json!({})), ("scale", Value::Null), ("freeze", json!(false))])
}

fn target(target: &mut Map<String, Value>) -> bool {
    fill(target, &[("scale", json!(1)), ("format", json!("rgba8"))])
}

fn lighter(lighter: &mut Map<String, Value>) -> bool {
    let mut changed = fill(lighter, &[("up_to", Value::Null)]);
    changed |= cost(lighter.get_mut("cost"));
    changed | each(lighter.get_mut("passes"), pass)
}

pub fn descriptor(descriptor: &mut Value) -> bool {
    let Some(d) = descriptor.as_object_mut() else { return false };
    let none = Value::Null;
    let mut changed = fill(
        d,
        &[
            ("name", json!({})),
            ("category", none.clone()),
            ("description", none.clone()),
            ("uses", json!([])),
            ("audio", json!([])),
            ("mask_mode", json!("mix")),
            ("params", json!([])),
            ("passes", json!([])),
            ("cost", json!({})),
            ("fuse", none.clone()),
            ("reach", none.clone()),
            ("bounded", json!([])),
            ("min_fps", none.clone()),
            ("velocity", none.clone()),
            ("state", none.clone()),
            ("solve", none.clone()),
            ("pace", none.clone()),
            ("quality", json!({})),
            ("targets", json!({})),
            ("options", json!([])),
            ("stage", json!("layer")),
            ("compose", json!(false)),
            ("lighter", none),
        ],
    );
    changed |= cost(d.get_mut("cost"));
    changed |= each(d.get_mut("params"), param);
    changed |= each(d.get_mut("passes"), pass);
    changed |= each(d.get_mut("options"), |v| fill(v, &[("when_bound", Value::Null)]));
    changed |= members(d.get_mut("quality"), tune);
    changed |= members(d.get_mut("targets"), target);
    if let Some(state) = d.get_mut("state").and_then(Value::as_object_mut) {
        changed |= fill(state, &[("scale", json!(1)), ("settle", json!(0)), ("fade", Value::Null)]);
    }
    if let Some(l) = d.get_mut("lighter").and_then(Value::as_object_mut) {
        changed |= lighter(l);
    }
    changed
}

fn members(value: Option<&mut Value>, mut step: impl FnMut(&mut Map<String, Value>) -> bool) -> bool {
    let items = value.and_then(Value::as_object_mut).into_iter().flat_map(|m| m.values_mut());
    items.filter_map(Value::as_object_mut).fold(false, |done, item| step(item) | done)
}
