use super::applied;
use super::walk::{Walk, nullable};
use serde_json::{Map, Value};

impl<'a> Walk<'a> {
    pub(super) fn hidden(
        &self,
        ps: &'a Value,
        required: bool,
        def: &str,
        ctx: &str,
        name: &str,
    ) -> Option<Value> {
        if ps.is_boolean() {
            return None;
        }
        if let Some(fixed) = super::particles::value(def, ctx, name) {
            return Some(fixed);
        }
        if let Some(fixed) = applied::corrected(def, name) {
            return Some(fixed);
        }
        if let Some(d) = ps.get("default").or_else(|| ps.get("x-follows")) {
            return Some(d.clone());
        }
        if nullable(ps) {
            return Some(Value::Null);
        }
        if required {
            return applied::value(def, name).or_else(|| applied::value(ctx, name));
        }
        let (target, _) = self.resolve(ps);
        match self.shape(target).as_slice() {
            ["object"] => Some(Value::Object(Map::new())),
            ["array"] => Some(Value::Array(Vec::new())),
            ["string"] => Some(Value::String(String::new())),
            ["boolean"] => Some(Value::Bool(false)),
            ["number"] => Some(Value::from(0)),
            _ => Some(Value::Object(Map::new())),
        }
    }
}
