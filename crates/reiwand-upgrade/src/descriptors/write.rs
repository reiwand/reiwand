use serde_json::Value;

fn inline(value: &Value, out: &mut String) {
    match value {
        Value::Object(map) => {
            out.push('{');
            let first = map.get_key_value("id").into_iter().chain(map.get_key_value("type"));
            let rest = map.iter().filter(|(k, _)| *k != "id" && *k != "type");
            for (at, (key, item)) in first.chain(rest).enumerate() {
                if at > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{key:?}: "));
                inline(item, out);
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            for (at, item) in items.iter().enumerate() {
                if at > 0 {
                    out.push_str(", ");
                }
                inline(item, out);
            }
            out.push(']');
        }
        other => out.push_str(&other.to_string()),
    }
}

fn spread(value: &Value, depth: usize, limit: usize, out: &mut String) {
    let pad = "  ".repeat(depth + 1);
    let rows: Vec<(Option<&String>, &Value)> = match value {
        Value::Object(map) if depth < limit && !map.is_empty() => {
            map.iter().map(|(k, v)| (Some(k), v)).collect()
        }
        Value::Array(items) if depth < limit && !items.is_empty() => {
            items.iter().map(|v| (None, v)).collect()
        }
        _ => return inline(value, out),
    };
    let (open, close) = if value.is_object() { ('{', '}') } else { ('[', ']') };
    out.push(open);
    out.push('\n');
    for (at, (key, item)) in rows.iter().enumerate() {
        out.push_str(&pad);
        if let Some(key) = key {
            out.push_str(&format!("{key:?}: "));
        }
        spread(item, depth + 1, limit, out);
        out.push_str(if at + 1 < rows.len() { ",\n" } else { "\n" });
    }
    out.push_str(&"  ".repeat(depth));
    out.push(close);
}

pub fn text(doc: &Value, limit: usize) -> String {
    let mut out = String::new();
    spread(doc, 0, limit, &mut out);
    out.push('\n');
    out
}
