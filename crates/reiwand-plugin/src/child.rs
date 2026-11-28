use crate::net::{Answer, Fault, Request};
use crate::runner::{Call, run};
use crate::{Error, Limits};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

pub const HOST_ARG: &str = "--provider-plugin-host";
const MAX_LINE: u64 = 16 << 20;
const GRACE: Duration = Duration::from_secs(2);

#[derive(Serialize, Deserialize)]
struct Start {
    modules: BTreeMap<String, String>,
    limits: Limits,
    call: Call,
}

#[derive(Serialize, Deserialize)]
enum Said {
    Fetch(Request),
    Done(Result<Value, Error>),
}

fn line<T: for<'de> Deserialize<'de>>(input: &mut impl BufRead) -> std::io::Result<Option<T>> {
    let mut buf = Vec::new();
    input.take(MAX_LINE).read_until(b'\n', &mut buf)?;
    if buf.is_empty() {
        return Ok(None);
    }
    serde_json::from_slice(&buf).map(Some).map_err(std::io::Error::other)
}

fn say(out: &mut impl Write, value: &impl Serialize) -> std::io::Result<()> {
    let mut text = serde_json::to_vec(value).map_err(std::io::Error::other)?;
    text.push(b'\n');
    out.write_all(&text)?;
    out.flush()
}

pub fn serve() -> std::io::Result<()> {
    reiwand_script::sandbox()?;
    let mut input = BufReader::new(std::io::stdin());
    let Some(start) = line::<Start>(&mut input)? else { return Ok(()) };
    let fetch = Box::new(move |request: &Request| {
        let mut out = std::io::stdout().lock();
        say(&mut out, &Said::Fetch(request.clone())).map_err(|e| Fault::Failed(e.to_string()))?;
        match line::<Result<Answer, Fault>>(&mut input) {
            Ok(Some(answer)) => answer,
            _ => Err(Fault::Refused("the service stopped answering".into())),
        }
    });
    let done = run(&start.modules, &start.limits, &start.call, fetch);
    say(&mut std::io::stdout().lock(), &Said::Done(done))
}

pub fn serve_if_asked() {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new(HOST_ARG)) {
        std::process::exit(if serve().is_ok() { 0 } else { 1 });
    }
}

pub(crate) fn run_in(
    program: &(PathBuf, Vec<String>),
    modules: &BTreeMap<String, String>,
    limits: &Limits,
    call: &Call,
    mut fetch: impl FnMut(&Request) -> Result<Answer, Fault>,
) -> Result<Value, Error> {
    let host = |e: std::io::Error| Error::Host(e.to_string());
    let mut command = Command::new(&program.0);
    command.args(&program.1).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
    let orphaned = || match unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) } {
        0 => Ok(()),
        _ => Err(std::io::Error::last_os_error()),
    };
    let mut child = unsafe { command.pre_exec(orphaned) }.spawn().map_err(host)?;
    let (Some(mut input), Some(output)) = (child.stdin.take(), child.stdout.take()) else {
        return Err(Error::Host("no pipes to the plugin host".into()));
    };
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut output = BufReader::new(output);
        while let Ok(Some(said)) = line::<Said>(&mut output) {
            if tx.send(said).is_err() {
                break;
            }
        }
    });
    let until = Instant::now() + limits.wall + GRACE;
    let start = Start { modules: modules.clone(), limits: *limits, call: call.clone() };
    let result = say(&mut input, &start).map_err(host).and_then(|()| {
        loop {
            match rx.recv_timeout(until.saturating_duration_since(Instant::now())) {
                Ok(Said::Fetch(request)) => say(&mut input, &fetch(&request)).map_err(host)?,
                Ok(Said::Done(done)) => break done,
                Err(mpsc::RecvTimeoutError::Timeout) => break Err(Error::Time(limits.wall)),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break Err(Error::Host("the plugin host stopped without an answer".into()));
                }
            }
        }
    });
    let _ = child.kill();
    let _ = child.wait();
    result
}
