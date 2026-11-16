use crate::args::{INVALID, fixture};
use crate::bus::Daemon;
use crate::methods::dispatch;
use crate::tests::private_bus;
use reiwand_client::{Client, parse};
use reiwand_core::dbus::{BUS_NAME, OBJECT_PATH};
use reiwand_core::model::{MODEL, State};
use serde_json::{Value, json};
use std::collections::HashMap;
use zbus::connection::Builder;

struct FakeDaemon;

#[zbus::interface(name = "io.github.reiwand.Daemon1")]
impl FakeDaemon {
    #[zbus(property(emits_changed_signal = "const"))]
    fn model(&self) -> u32 {
        MODEL
    }

    fn get_state(&self) -> String {
        fixture("Daemon1", "GetState")
    }
}

struct FakeLibrary;

#[zbus::interface(name = "io.github.reiwand.Library1")]
impl FakeLibrary {
    fn add_source(&self, path: &str, recursive: bool) -> (String, u64) {
        (format!("{path} {recursive}"), 4)
    }

    fn set_tags(&self, uids: Vec<String>, add: Vec<String>, remove: Vec<String>) -> zbus::fdo::Result<()> {
        match uids.len() + add.len() + remove.len() {
            3 => Ok(()),
            _ => Err(zbus::fdo::Error::InvalidArgs("expected one of each".into())),
        }
    }

    fn request_thumbnails(&self, uids: Vec<String>, size: u32) -> HashMap<String, String> {
        uids.into_iter().map(|uid| (uid, format!("/tmp/{size}.jpg"))).collect()
    }
}

#[tokio::test]
async fn calls_reach_the_daemon_with_typed_arguments() {
    let (mut bus, _dir, address) = private_bus();
    let service = Builder::address(address.trim()).name(BUS_NAME);
    let service = service.serve_at(OBJECT_PATH, FakeDaemon).serve_at(OBJECT_PATH, FakeLibrary);
    let _service = service.build().await.unwrap();
    let client = Client::new(Builder::address(address.trim()).build().await.unwrap()).await.unwrap();
    let daemon = Daemon::Bus(client);
    let call =
        |iface, member, args: Value| dispatch(&daemon, iface, member, args.as_array().unwrap().clone());
    assert_eq!(call("Library1", "AddSource", json!(["/tmp/x", true])).await.unwrap(), r#"["/tmp/x true",4]"#);
    let thumbs = call("Library1", "RequestThumbnails", json!([["A"], 512])).await.unwrap();
    assert_eq!(thumbs, r#"[{"A":"/tmp/512.jpg"}]"#);
    assert_eq!(call("Library1", "SetTags", json!([["A"], ["sea"], ["old"]])).await.unwrap(), "[]");
    let state: State = parse(&call("Daemon1", "GetState", json!([])).await.unwrap()).unwrap();
    assert_eq!(state.outputs.len(), 2);
    let failed = call("Library1", "SetTags", json!([[], [], []])).await.unwrap_err();
    assert_eq!((failed.name.as_str(), failed.message.as_str()), (INVALID, "expected one of each"));
    let older = call("Playlists1", "List", json!([])).await.unwrap_err();
    assert_eq!(older.message, "reiwandd is older than this client; install matching versions of Reiwand");
    bus.kill().ok();
    bus.wait().ok();
}
