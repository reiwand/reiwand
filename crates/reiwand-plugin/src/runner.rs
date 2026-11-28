use crate::bundle::MAIN;
use crate::modules::Modules;
use crate::net::{Answer, Fault, Request};
use crate::{Error, Limits};
use rquickjs::promise::MaybePromise;
use rquickjs::{CatchResultExt, Context, Ctx, Exception, Function, Module, Object, Runtime};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

const PRELUDE: &str = include_str!("prelude.js");
const MAX_RESULT: usize = 4 << 20;

pub type Fetcher = Box<dyn FnMut(&Request) -> Result<Answer, Fault>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Call {
    pub settings: Map<String, Value>,
    pub page: Option<String>,
}

fn cpu() -> Duration {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut t) };
    Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
}

type Stop = Rc<RefCell<Option<Error>>>;

fn stopped(stop: &Stop, error: Error) {
    stop.borrow_mut().get_or_insert(error);
}

pub fn run(
    modules: &BTreeMap<String, String>,
    limits: &Limits,
    call: &Call,
    fetch: Fetcher,
) -> Result<Value, Error> {
    let failed = |e: rquickjs::Error| Error::Host(e.to_string());
    let runtime = Runtime::new().map_err(failed)?;
    runtime.set_memory_limit(limits.memory);
    runtime.set_max_stack_size(limits.stack);
    let loader = Modules(modules.clone());
    runtime.set_loader(loader.clone(), loader);
    let (stop, start, spent) = (Stop::default(), Instant::now(), cpu());
    let (watch, limits) = (stop.clone(), *limits);
    runtime.set_interrupt_handler(Some(Box::new(move || {
        if cpu().saturating_sub(spent) > limits.cpu {
            stopped(&watch, Error::Cpu(limits.cpu));
        } else if start.elapsed() > limits.wall {
            stopped(&watch, Error::Time(limits.wall));
        }
        watch.borrow().is_some()
    })));
    let context = Context::full(&runtime).map_err(failed)?;
    let text = context.with(|ctx| {
        let raw = host(&ctx, fetch, &stop, limits.fetches);
        raw.and_then(|raw| listed(&ctx, call, raw)).catch(&ctx).map_err(|e| e.to_string())
    });
    if let Some(error) = stop.take() {
        return Err(error);
    }
    let text = text.map_err(|why| match why.contains("out of memory") {
        true => Error::Memory,
        false => Error::Script(why.chars().take(1024).collect()),
    })?;
    let text = text.ok_or_else(|| Error::Page("list() returned no value".into()))?;
    if text.len() > MAX_RESULT {
        return Err(Error::Limit(format!("list() returned more than {} KiB", MAX_RESULT >> 10)));
    }
    serde_json::from_str(&text).map_err(|e| Error::Page(e.to_string()))
}

fn host<'js>(ctx: &Ctx<'js>, fetch: Fetcher, stop: &Stop, most: u32) -> rquickjs::Result<Function<'js>> {
    let (fetch, stop, count) = (Rc::new(RefCell::new(fetch)), stop.clone(), Rc::new(Cell::new(0u32)));
    Function::new(ctx.clone(), move |ctx: Ctx<'js>, asked: String| -> rquickjs::Result<String> {
        count.set(count.get() + 1);
        if count.get() > most {
            let why = format!("the plugin made more than {most} requests in one call");
            stopped(&stop, Error::Limit(why.clone()));
            return Err(Exception::throw_message(&ctx, &why));
        }
        let request: Request = serde_json::from_str(&asked)
            .map_err(|e| Exception::throw_type(&ctx, &format!("fetch: invalid arguments: {e}")))?;
        let answered = fetch.borrow_mut()(&request);
        match answered {
            Ok(answer) => Ok(serde_json::to_string(&answer).unwrap_or_default()),
            Err(Fault::Failed(why)) => Err(Exception::throw_message(&ctx, &format!("fetch failed: {why}"))),
            Err(Fault::Refused(why)) => {
                stopped(&stop, Error::Refused(why.clone()));
                Err(Exception::throw_message(&ctx, &why))
            }
            Err(Fault::Limit(why)) => {
                stopped(&stop, Error::Limit(why.clone()));
                Err(Exception::throw_message(&ctx, &why))
            }
        }
    })
}

fn listed<'js>(ctx: &Ctx<'js>, call: &Call, raw: Function<'js>) -> rquickjs::Result<Option<String>> {
    let install: Function = ctx.eval(PRELUDE)?;
    install.call::<_, ()>((raw,))?;
    let names: Object = Module::import(ctx, MAIN)?.finish()?;
    let list: Function =
        names.get("list").map_err(|_| Exception::throw_message(ctx, "main.ts exports no function list"))?;
    let given = ctx.json_parse(serde_json::to_string(call).unwrap_or_default())?;
    let out: MaybePromise = list.call((given,))?;
    let value: rquickjs::Value = out.finish().map_err(|e| match e {
        rquickjs::Error::WouldBlock => {
            Exception::throw_message(ctx, "list() waits for something that never finishes")
        }
        e => e,
    })?;
    ctx.json_stringify(value)?.map(|s| s.to_string()).transpose()
}
