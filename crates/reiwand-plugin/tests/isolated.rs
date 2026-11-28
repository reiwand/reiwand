mod common;
use reiwand_plugin::{Error, HOST_ARG, Isolation, Limits, Plugin};
use std::collections::BTreeMap;
use std::process::ExitCode;
use std::time::Duration;

const MAIN: &str = r#"
export async function list({ page }: { page: string | null }) {
  if (page === "loop") for (;;) {}
  const answer = await fetch("https://api.example.test/one");
  const one = await answer.json();
  return { items: [{ ...one, page: null, author: null, license: null }], next: null };
}
"#;

fn route(head: &str) -> (u16, Vec<(String, String)>, String) {
    let one = r#"{"id": "1", "title": "One", "media": "video", "url": "https://api.example.test/1.webm"}"#;
    match head.starts_with("GET /one ") {
        true => (200, Vec::new(), one.into()),
        false => (404, Vec::new(), String::new()),
    }
}

fn isolation() -> Isolation {
    Isolation::Process(std::env::current_exe().unwrap(), vec![HOST_ARG.into()])
}

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(HOST_ARG) {
        return reiwand_plugin::serve().map_or(ExitCode::FAILURE, |()| ExitCode::SUCCESS);
    }
    reiwand_core::net::set_private(false);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let server = runtime.block_on(common::serve(route));
    let manifest = common::manifest(&["api.example.test"], "null", "[]");
    let (_root, installed) = common::installed(&manifest, &[("main.ts", MAIN)]);
    let net = server.net(&installed.manifest, None);
    let mut plugin = Plugin::new(installed);
    let page = plugin.list(net.clone(), Default::default(), None, &isolation()).unwrap();
    assert_eq!(page.items[0].url, "https://api.example.test/1.webm");
    println!("test a_plugin_in_its_own_process_lists_through_the_host ... ok");
    plugin.limits = Limits { cpu: Duration::from_millis(300), ..Limits::STANDARD };
    let stopped = plugin.list(net, Default::default(), Some("loop".into()), &isolation());
    assert_eq!(stopped, Err(Error::Cpu(Duration::from_millis(300))));
    println!("test a_loop_in_its_own_process_is_stopped ... ok");
    let bare = BTreeMap::new();
    assert!(plugin.settings(&bare).unwrap().is_empty());
    ExitCode::SUCCESS
}
