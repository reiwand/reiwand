use crate::value::{Binding, Value};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Spread<T> {
    pub ends: [T; 2],
    pub exponent: f32,
    pub bound: Option<Box<[Value<T>; 2]>>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
enum SpreadRepr<T> {
    Fixed(T, T, f32),
    Bound(Value<T>, Value<T>, f32),
}

impl<T> From<[T; 2]> for Spread<T> {
    fn from(ends: [T; 2]) -> Self {
        Spread { ends, exponent: 1.0, bound: None }
    }
}

impl<T> std::ops::Index<usize> for Spread<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.ends[i]
    }
}

impl<T: Clone + Serialize> Serialize for Spread<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.bound.as_deref().cloned() {
            Some([a, b]) => SpreadRepr::Bound(a, b, self.exponent).serialize(s),
            None => {
                let [a, b] = self.ends.clone();
                SpreadRepr::Fixed(a, b, self.exponent).serialize(s)
            }
        }
    }
}

impl<'de, T: Deserialize<'de> + Clone + Default> Deserialize<'de> for Spread<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let fixed = |v: &Value<T>| match v {
            Value::Fixed(v) => v.clone(),
            Value::Bound(_) => T::default(),
        };
        Ok(match SpreadRepr::deserialize(d)? {
            SpreadRepr::Fixed(a, b, exponent) => Spread { ends: [a, b], exponent, bound: None },
            SpreadRepr::Bound(a, b, exponent) => {
                Spread { ends: [fixed(&a), fixed(&b)], exponent, bound: Some(Box::new([a, b])) }
            }
        })
    }
}

impl<T: JsonSchema> JsonSchema for Spread<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("Spread_for_{}", T::schema_name()).into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut schema = SpreadRepr::<T>::json_schema(generator);
        let branches = schema.get_mut("anyOf").and_then(serde_json::Value::as_array_mut);
        for branch in branches.into_iter().flatten() {
            if let Some(exponent) =
                branch.pointer_mut("/prefixItems/2").and_then(serde_json::Value::as_object_mut)
            {
                exponent.insert("default".into(), serde_json::json!(1.0));
            }
        }
        schema
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "Value<f32>", into = "Value<f32>")]
pub struct Num(pub f32, pub Option<Box<Binding<f32>>>);

impl From<Value<f32>> for Num {
    fn from(v: Value<f32>) -> Num {
        match v {
            Value::Fixed(x) => Num(x, None),
            Value::Bound(b) => Num(0.0, Some(b)),
        }
    }
}

impl From<Num> for Value<f32> {
    fn from(n: Num) -> Value<f32> {
        n.1.map_or(Value::Fixed(n.0), Value::Bound)
    }
}

impl From<f32> for Num {
    fn from(x: f32) -> Num {
        Num(x, None)
    }
}

impl JsonSchema for Num {
    fn schema_name() -> Cow<'static, str> {
        Value::<f32>::schema_name()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        Value::<f32>::json_schema(generator)
    }
}
