use super::{Customs, Report, params};
use serde_json::{Map, Value};

pub(super) struct Walk<'a> {
    pub(super) root: &'a Value,
    customs: &'a Customs,
    required_only: bool,
    pub report: Report,
}

fn kind(v: &Value) -> &'static str {
    match v {
        Value::Object(_) => "object",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::Null => "null",
    }
}

const KINDS: [&str; 6] = ["object", "array", "string", "boolean", "number", "null"];

fn canon(name: &str) -> Option<&'static str> {
    let name = if name == "integer" { "number" } else { name };
    KINDS.iter().copied().find(|k| *k == name)
}

pub(super) fn nullable(s: &Value) -> bool {
    let typed = s
        .get("type")
        .is_some_and(|t| t == "null" || t.as_array().is_some_and(|a| a.iter().any(|x| x == "null")));
    typed
        || s.get("x-null") == Some(&Value::Bool(true))
        || s.get("anyOf")
            .and_then(Value::as_array)
            .is_some_and(|a| a.iter().any(|b| b.get("type") == Some(&Value::from("null"))))
}

impl<'a> Walk<'a> {
    pub fn new(root: &'a Value, customs: &'a Customs, required_only: bool) -> Walk<'a> {
        Walk { root, customs, required_only, report: Report::default() }
    }

    pub fn start(&mut self, doc: &mut Value) {
        let root = self.root;
        let title = root.get("title").and_then(Value::as_str).unwrap_or("");
        self.visit(root, doc, title, "");
    }

    pub(super) fn resolve(&self, s: &'a Value) -> (&'a Value, Option<&'a str>) {
        let Some(name) = s.get("$ref").and_then(Value::as_str).and_then(|r| r.rsplit('/').next()) else {
            return (s, None);
        };
        match self.root.get("$defs").and_then(|d| d.get(name)) {
            Some(target) => (target, Some(name)),
            None => (s, None),
        }
    }

    pub(super) fn shape(&self, s: &'a Value) -> Vec<&'static str> {
        let (s, _) = self.resolve(s);
        if let Some(t) = s.get("type") {
            let names = t.as_array().map_or_else(
                || t.as_str().into_iter().collect(),
                |a| a.iter().filter_map(Value::as_str).collect::<Vec<_>>(),
            );
            return names.into_iter().filter_map(canon).collect();
        }
        for key in ["oneOf", "anyOf"] {
            if let Some(list) = s.get(key).and_then(Value::as_array) {
                let mut all: Vec<&'static str> = list.iter().flat_map(|b| self.shape(b)).collect();
                all.dedup();
                return all;
            }
        }
        if s.get("properties").is_some() || s.get("additionalProperties").is_some() {
            return vec!["object"];
        }
        if s.get("items").is_some() || s.get("prefixItems").is_some() {
            return vec!["array"];
        }
        KINDS.to_vec()
    }

    fn penalty(&self, branch: &'a Value, map: &Map<String, Value>) -> Option<usize> {
        let (b, _) = self.resolve(branch);
        if b.get("oneOf").is_some() || b.get("anyOf").is_some() {
            let list = b.get("oneOf").or_else(|| b.get("anyOf")).and_then(Value::as_array)?;
            return list.iter().filter_map(|x| self.penalty(x, map)).min();
        }
        let props = b.get("properties").and_then(Value::as_object);
        let mut cost = 0;
        for (name, p) in props.into_iter().flatten() {
            let tag = p.get("const");
            match (tag, map.get(name)) {
                (Some(t), Some(v)) if t != v => return None,
                (Some(_), None) => return None,
                _ => {}
            }
        }
        let extra = b.get("additionalProperties");
        let closed = extra == Some(&Value::Bool(false));
        let mapped = extra.is_some_and(Value::is_object);
        for key in map.keys() {
            if props.is_none_or(|p| !p.contains_key(key)) && !mapped {
                cost += if closed { 10 } else { 1 };
            }
        }
        if let Some(req) = b.get("required").and_then(Value::as_array) {
            cost += req.iter().filter_map(Value::as_str).filter(|r| !map.contains_key(*r)).count();
        }
        Some(cost)
    }

    fn pick(&self, branches: &'a [Value], v: &Value) -> Option<&'a Value> {
        let want = kind(v);
        let fits = branches.iter().filter(|b| self.shape(b).contains(&want));
        if let Value::Object(map) = v {
            return fits
                .filter_map(|b| self.penalty(b, map).map(|c| (c, b)))
                .min_by_key(|(c, _)| *c)
                .map(|(_, b)| b);
        }
        fits.into_iter().next()
    }

    fn visit(&mut self, s: &'a Value, v: &mut Value, def: &str, ctx: &str) {
        let (s, named) = self.resolve(s);
        let def = named.unwrap_or(def);
        super::particles::spread(def, v);
        if let Value::Object(map) = v
            && let Some((name, value)) = super::legacy::tag(ctx, map)
        {
            map.insert(name.into(), value);
            self.report.note("Refract", name);
        }
        for key in ["oneOf", "anyOf"] {
            if let Some(list) = s.get(key).and_then(Value::as_array)
                && let Some(branch) = self.pick(list, v)
            {
                self.visit(branch, v, def, ctx);
            }
        }
        for part in s.get("allOf").and_then(Value::as_array).into_iter().flatten() {
            self.visit(part, v, def, ctx);
        }
        match v {
            Value::Object(map) => self.object(s, map, def, ctx),
            Value::Array(list) => {
                if let Some(items) = s.get("items").filter(|i| i.is_object()) {
                    for one in list {
                        self.visit(items, one, "", ctx);
                    }
                }
            }
            _ => {}
        }
    }

    fn object(&mut self, s: &'a Value, map: &mut Map<String, Value>, def: &str, ctx: &str) {
        if def == "EffectUse" && !self.required_only {
            params::complete(map, self.customs, &mut self.report);
        }
        for (name, value) in super::legacy::heard(def, map) {
            map.insert(name.into(), value);
            self.report.note(def, name);
        }
        super::particles::before(ctx, map);
        let props = s.get("properties").and_then(Value::as_object);
        let required =
            |n: &str| s.get("required").and_then(Value::as_array).is_some_and(|r| r.iter().any(|x| x == n));
        for (name, ps) in props.into_iter().flatten() {
            if !map.contains_key(name)
                && (!self.required_only || required(name))
                && let Some(value) = self.hidden(ps, required(name), def, ctx, name)
            {
                map.insert(name.clone(), value);
                self.report.note(if def.is_empty() { ctx } else { def }, name);
            }
            if map.get(name).is_none_or(Value::is_null)
                && let Some(word) = super::legacy::follows(def, name, map)
            {
                map.insert(name.clone(), word);
                self.report.note(def, name);
            }
            if let Some(child) = map.get_mut(name) {
                self.visit(ps, child, "", name);
            }
        }
        if let Some(extra) = s.get("additionalProperties").filter(|a| a.is_object()) {
            for (key, child) in map.iter_mut() {
                if props.is_none_or(|p| !p.contains_key(key)) {
                    self.visit(extra, child, "", key);
                }
            }
        }
    }
}
