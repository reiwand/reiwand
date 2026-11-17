use super::legacy;
use super::walk::{Walk, nullable};
use serde_json::{Map, Value};

impl<'a> Walk<'a> {
    pub fn fallback(&self, ps: &'a Value, required: bool, def: &str, name: &str) -> Option<Value> {
        self.hidden(ps, required, def, def, name)
    }

    pub fn describe(&self, ps: &'a Value) -> String {
        let (target, named) = self.resolve(ps);
        named.map_or_else(|| self.shape(target).join("|"), str::to_string)
    }

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
        if let Some(fixed) = legacy::corrected(def, name) {
            return Some(fixed);
        }
        if let Some(d) = ps.get("default") {
            return Some(d.clone());
        }
        if nullable(ps) {
            return Some(Value::Null);
        }
        if required {
            return legacy::value(def, name).or_else(|| legacy::value(ctx, name));
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
