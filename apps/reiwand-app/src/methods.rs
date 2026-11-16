use crate::args::{MAX_PAGE, fail, fixture, invalid, next};
use crate::bus::Daemon;
use reiwand_client::Error;
use reiwand_core::dbus::{BUS_NAME, OBJECT_PATH};
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::LazyLock;
use zbus::wire::{DynamicType, Structure, StructureBuilder, Variant};

pub struct Method {
    pub args: Vec<(String, String)>,
    pub reply: Vec<(String, String)>,
}

pub static METHODS: LazyLock<HashMap<String, Method>> =
    LazyLock::new(|| documented(include_str!("../../../docs/specs/dbus.md")));

fn documented(doc: &str) -> HashMap<String, Method> {
    let mut methods = HashMap::new();
    for section in doc.split("\n## `io.github.reiwand.").skip(1) {
        let iface = section.split('`').next().unwrap_or_default();
        for heading in section.split("\n## ").next().unwrap_or_default().split("\n### `").skip(1) {
            let heading = heading.split('`').next().unwrap_or_default();
            let Some((member, rest)) = heading.split_once('(') else { continue };
            let Some((args, reply)) = rest.split_once(')') else { continue };
            let reply =
                reply.trim().trim_start_matches('→').trim().trim_start_matches('(').trim_end_matches(')');
            let method = Method { args: pairs(args), reply: pairs(reply) };
            methods.insert(format!("{iface}.{member}"), method);
        }
    }
    methods
}

fn pairs(list: &str) -> Vec<(String, String)> {
    let pair = |a: &str| a.trim().split_once(' ').map(|(s, n)| (s.to_owned(), n.trim().to_owned()));
    list.split(',').filter_map(pair).collect()
}

pub async fn dispatch(daemon: &Daemon, iface: &str, member: &str, args: Vec<Value>) -> Result<String, Error> {
    if let (Daemon::Fixtures, "Library1", "Search") = (daemon, iface, member)
        && let Some(reply) = crate::crowd::search(&args)
    {
        return Ok(reply);
    }
    let method = METHODS.get(&format!("{iface}.{member}"));
    let method = method.ok_or_else(|| invalid(format!("unknown method {iface}.{member}")))?;
    let limit = |((_, name), arg): (&(String, String), &Value)| {
        name == "limit" && arg.as_u64().is_some_and(|n| n > MAX_PAGE)
    };
    let paged = method.args.iter().zip(&args).any(limit);
    let mut values = args.into_iter();
    let mut body = StructureBuilder::new();
    for (signature, name) in &method.args {
        let values = &mut values;
        body = match signature.as_str() {
            "s" => body.add_field(next::<String>(values, name)?),
            "b" => body.add_field(next::<bool>(values, name)?),
            "u" => body.add_field(next::<u32>(values, name)?),
            "i" => body.add_field(next::<i32>(values, name)?),
            "t" => body.add_field(next::<u64>(values, name)?),
            "d" => body.add_field(next::<f64>(values, name)?),
            "as" => body.add_field(next::<Vec<String>>(values, name)?),
            "au" => body.add_field(next::<Vec<u32>>(values, name)?),
            "ay" => body.add_field(next::<Vec<u8>>(values, name)?),
            "a{ss}" => body.add_field(next::<HashMap<String, String>>(values, name)?),
            other => return Err(invalid(format!("{name}: cannot send {other}"))),
        };
    }
    if values.len() > 0 {
        return Err(invalid(format!("{member} takes fewer arguments")));
    }
    let client = match daemon {
        Daemon::Bus(client) => client,
        Daemon::Fixtures if paged => return Err(invalid(format!("limit must be at most {MAX_PAGE}"))),
        Daemon::Fixtures => return Ok(fixture(iface, member)),
    };
    let iface = format!("io.github.reiwand.{iface}");
    let reply = match method.args.is_empty() {
        true => send(&client.bus, &iface, member, &()).await,
        false => send(&client.bus, &iface, member, &body.build().map_err(fail)?).await,
    };
    let reply = reply.map_err(fail)?;
    let body = reply.body();
    if method.reply == [("s".to_owned(), "json".to_owned())] {
        return body.deserialize::<String>().map_err(fail);
    }
    if body.signature().to_string().is_empty() {
        return Ok("[]".into());
    }
    let fields = body.deserialize::<Structure>().map_err(fail)?;
    Ok(Value::Array(fields.fields().iter().map(to_json).collect()).to_string())
}

async fn send<B: Serialize + DynamicType>(
    bus: &zbus::Connection,
    iface: &str,
    member: &str,
    body: &B,
) -> zbus::Result<zbus::Message> {
    bus.call_method(Some(BUS_NAME), OBJECT_PATH, Some(iface), member, body).await
}

fn to_json(value: &Variant) -> Value {
    match value {
        Variant::U8(n) => json!(n),
        Variant::Bool(b) => json!(b),
        Variant::I32(n) => json!(n),
        Variant::U32(n) => json!(n),
        Variant::I64(n) => json!(n),
        Variant::U64(n) => json!(n),
        Variant::F64(n) => json!(n),
        Variant::Str(s) => json!(s.as_str()),
        Variant::Array(a) => Value::Array(a.inner().iter().map(to_json).collect()),
        Variant::Structure(s) => Value::Array(s.fields().iter().map(to_json).collect()),
        Variant::Dict(d) => {
            let key = |k: &Variant| to_json(k).as_str().map_or_else(|| to_json(k).to_string(), str::to_owned);
            Value::Object(d.iter().map(|(k, v)| (key(k), to_json(v))).collect())
        }
        _ => Value::Null,
    }
}
