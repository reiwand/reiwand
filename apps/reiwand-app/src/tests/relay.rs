use crate::bus::Daemon;
use crate::methods::{METHODS, dispatch};
use crate::tests::private_bus;
use futures_util::StreamExt;
use reiwand_client::Client;
use reiwand_core::dbus::BUS_NAME;
use reiwand_core::model::MODEL;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::connection::Builder;
use zbus::message::Type;
use zbus::wire::{StructureBuilder, Variant};

type Seen = Arc<Mutex<Vec<(String, String, Vec<u8>)>>>;

fn sample(signature: &str, full: bool) -> Value {
    match (signature, full) {
        ("s", _) => json!(if full { "text \u{e9}" } else { "" }),
        ("b", _) => json!(full),
        ("u", true) | ("t", true) => json!(7),
        ("i", true) => json!(-3),
        ("d", true) => json!(1.5),
        ("u" | "t" | "i" | "d", false) => json!(0),
        ("as", true) => json!(["a", "b"]),
        ("au" | "ay", true) => json!([1, 2, 255]),
        ("a{ss}", true) => json!({"k": "v"}),
        ("a{ss}", false) => json!({}),
        _ => json!([]),
    }
}

fn reply_body(signature: &str, full: bool) -> Variant<'static> {
    let value = sample(signature, full);
    match signature {
        "s" => Variant::from(value.as_str().unwrap().to_owned()),
        "b" => Variant::from(value.as_bool().unwrap()),
        "u" => Variant::from(value.as_u64().unwrap() as u32),
        "t" => Variant::from(value.as_u64().unwrap()),
        "as" => Variant::from(serde_json::from_value::<Vec<String>>(value).unwrap()),
        "ay" => Variant::from(serde_json::from_value::<Vec<u8>>(value).unwrap()),
        "a{ss}" => Variant::from(serde_json::from_value::<HashMap<String, String>>(value).unwrap()),
        other => panic!("no sample reply for {other}"),
    }
}

async fn serve(service: zbus::Connection, seen: Seen, full: bool) {
    let mut stream = zbus::MessageStream::from(&service);
    while let Some(Ok(message)) = stream.next().await {
        let header = message.header();
        if header.message_type() != Type::MethodCall {
            continue;
        }
        let iface = header.interface().map(|i| i.to_string()).unwrap_or_default();
        let member = header.member().map(|m| m.to_string()).unwrap_or_default();
        if iface == "org.freedesktop.DBus.Properties" {
            let model: HashMap<String, Variant> = [("Model".to_owned(), Variant::from(MODEL))].into();
            match member.as_str() {
                "Get" => service.reply(&header, &Variant::from(MODEL)).await.unwrap(),
                _ => service.reply(&header, &model).await.unwrap(),
            }
            continue;
        }
        let short = iface.trim_start_matches("io.github.reiwand.");
        let body = message.body();
        seen.lock().unwrap().push((
            format!("{short}.{member}"),
            body.signature().to_string(),
            body.data().to_vec(),
        ));
        let method = &METHODS[&format!("{short}.{member}")];
        let fields = method.reply.iter().map(|(s, _)| reply_body(s, full));
        match method.reply.is_empty() {
            true => service.reply(&header, &()).await.unwrap(),
            false => {
                let built = fields.fold(StructureBuilder::new(), StructureBuilder::append_field);
                service.reply(&header, &built.build().unwrap()).await.unwrap()
            }
        }
    }
}

#[tokio::test]
async fn every_documented_method_goes_out_with_its_signature_and_comes_back_as_json() {
    let (mut bus, _dir, address) = private_bus();
    for full in [true, false] {
        let service = Builder::address(address.trim()).name(BUS_NAME).build().await.unwrap();
        let seen = Seen::default();
        let server = tokio::spawn(serve(service, seen.clone(), full));
        let client = Client::new(Builder::address(address.trim()).build().await.unwrap()).await.unwrap();
        let daemon = Daemon::Bus(client);
        for (name, method) in METHODS.iter() {
            let (iface, member) = name.split_once('.').unwrap();
            let args: Vec<Value> = method.args.iter().map(|(s, _)| sample(s, full)).collect();
            let reply = dispatch(&daemon, iface, member, args.clone()).await.unwrap();
            let expected = match method.reply.as_slice() {
                [(s, n)] if s == "s" && n == "json" => sample(s, full).as_str().unwrap().to_owned(),
                reply => Value::Array(reply.iter().map(|(s, _)| sample(s, full)).collect()).to_string(),
            };
            assert_eq!(reply, expected, "{name}");
            let (sent, signature, _) = seen.lock().unwrap().pop().unwrap();
            let joined: String = method.args.iter().map(|(s, _)| s.as_str()).collect();
            let documented = if method.args.len() > 1 { format!("({joined})") } else { joined };
            assert_eq!((sent.as_str(), signature), (name.as_str(), documented));
            let longer = [args, vec![json!(1)]].concat();
            let longer = dispatch(&daemon, iface, member, longer).await.unwrap_err().message;
            assert_eq!(longer, format!("{member} takes fewer arguments"));
        }
        assert!(seen.lock().unwrap().is_empty());
        server.abort();
    }
    let loud = dispatch(&Daemon::Fixtures, "Daemon1", "SetVolume", vec![json!("loud")]).await.unwrap_err();
    assert_eq!(loud.message, "volume: invalid type: string \"loud\", expected f64");
    let paged =
        dispatch(&Daemon::Fixtures, "Library1", "Search", vec![json!(""), json!(0), json!(501)]).await;
    assert_eq!(paged.unwrap_err().message, "limit must be at most 500");
    let unknown = dispatch(&Daemon::Fixtures, "Library1", "Nothing", vec![]).await.unwrap_err();
    assert_eq!(unknown.message, "unknown method Library1.Nothing");
    bus.kill().ok();
    bus.wait().ok();
}
