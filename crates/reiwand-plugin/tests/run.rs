use reiwand_plugin::{Answer, Call, Error, Fault, Limits, Request, run};
use serde_json::{Map, Value, json};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Duration;

fn quick() -> Limits {
    Limits {
        cpu: Duration::from_millis(200),
        wall: Duration::from_secs(5),
        memory: 8 << 20,
        ..Limits::STANDARD
    }
}

fn call() -> Call {
    let mut settings = Map::new();
    settings.insert("query".into(), json!("cats"));
    Call { settings, page: None }
}

fn answer(body: &str) -> Answer {
    Answer {
        status: 200,
        url: "https://api.example.test/".into(),
        headers: BTreeMap::new(),
        body: body.into(),
    }
}

fn ran(main: &str, fetch: impl FnMut(&Request) -> Result<Answer, Fault> + 'static) -> Result<Value, Error> {
    let modules = BTreeMap::from([
        ("main.ts".to_owned(), main.to_owned()),
        ("lib/a.ts".into(), "export const a = 1;".into()),
    ]);
    run(&modules, &quick(), &call(), Box::new(fetch))
}

fn offline(main: &str) -> Result<Value, Error> {
    ran(main, |_| Err(Fault::Failed("offline".into())))
}

#[test]
fn list_gets_its_settings_and_page_and_may_import_its_modules() {
    let out = offline(
        "import { a } from './lib/a.ts';\nexport async function list({ settings, page }) {\n\
        return { a, query: settings.query, page }; }",
    );
    assert_eq!(out.unwrap(), json!({"a": 1, "query": "cats", "page": null}));
    let out = offline("import x from 'fs';\nexport const list = async () => 1;");
    assert!(matches!(&out, Err(Error::Script(why)) if why.contains("relative path")), "{out:?}");
}

#[test]
fn a_plugin_sees_no_files_processes_or_modules_of_the_host() {
    let out = offline(
        "export const list = async () => ['require', 'process', 'std', 'os', 'Deno', 'XMLHttpRequest']\n\
        .filter((name) => typeof globalThis[name] !== 'undefined');",
    );
    assert_eq!(out.unwrap(), json!([]));
}

#[test]
fn loops_and_memory_hogs_are_stopped() {
    let out = offline("export const list = async () => { for (;;) {} };");
    assert_eq!(out, Err(Error::Cpu(Duration::from_millis(200))));
    let out = offline(
        "export const list = async () => { const a = []; for (;;) a.push('x'.repeat(1 << 16) + a.length); };",
    );
    assert_eq!(out, Err(Error::Memory));
    let out = offline("export const list = () => new Promise(() => {});");
    assert!(matches!(&out, Err(Error::Script(why)) if why.contains("never finishes")), "{out:?}");
}

#[test]
fn fetch_answers_and_a_refusal_cannot_be_swallowed() {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    let out = ran(
        "export async function list() {\n const r = await fetch('https://api.example.test/x', \
         { method: 'post', headers: { Accept: 'application/json' }, body: 'q' });\n\
         return { ok: r.ok, data: await r.json(), type: r.headers.get('Nope') }; }",
        move |r| {
            log.borrow_mut().push(r.clone());
            Ok(answer("{\"n\": 3}"))
        },
    );
    assert_eq!(out.unwrap(), json!({"ok": true, "data": {"n": 3}, "type": null}));
    let asked = &seen.borrow()[0];
    assert_eq!((asked.method.as_str(), asked.body.as_deref()), ("POST", Some("q")));
    assert_eq!(asked.headers, [("Accept".to_owned(), "application/json".to_owned())]);
    let out = ran(
        "export async function list() { try { await fetch('https://elsewhere.test/'); } catch (e) {} return 1; }",
        |_| Err(Fault::Refused("not declared".into())),
    );
    assert_eq!(out, Err(Error::Refused("not declared".into())));
    let out = ran(
        "export async function list() { try { await fetch('https://api.example.test/'); } catch (e) { return String(e.message); } }",
        |_| Err(Fault::Failed("offline".into())),
    );
    assert_eq!(out.unwrap(), json!("fetch failed: offline"));
}

#[test]
fn too_many_requests_stop_the_call() {
    let out = ran(
        "export async function list() { for (;;) { try { await fetch('https://api.example.test/'); } catch (e) {} } }",
        |_| Ok(answer("")),
    );
    assert!(matches!(&out, Err(Error::Limit(why)) if why.contains("32 requests")), "{out:?}");
}
