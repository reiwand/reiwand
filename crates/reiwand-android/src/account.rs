use crate::state::CANCEL;
use crate::{Error, said};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Mutex;
use std::sync::atomic::Ordering::Relaxed;

mod book;
pub(crate) mod pins;
mod signin;
pub(crate) mod using;

static BUSY: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Request {
    Cancel,
    AddRepo { url: String, trust: Option<String> },
    ServerRepos { url: String, trust: Option<String> },
    Accounts,
    SignIn { url: String },
    SignInWait,
    UseAccount { url: String, account: String },
    UnlinkAccount { url: String },
    RemoveAccount { account: String },
    Repos,
    RemoveRepo { repo_id: String },
    Browse { query: String },
    Install { repo_id: String, id: String },
    Thumb { spec: String },
    InstallBase { uid: String },
    Votes { ids: Vec<String> },
    Vote { id: String, vote: String },
}

pub(crate) fn request(text: &str) -> String {
    let answer = crate::reply::tried(text, crate::updates::request)
        .or_else(|| crate::reply::tried(text, crate::providers::request))
        .or_else(|| crate::reply::tried(text, crate::carry::request))
        .unwrap_or_else(|| serde_json::from_str(text).map_err(Error::from).and_then(run));
    answer.unwrap_or_else(|e| e.reply()).to_string()
}

fn run(request: Request) -> Result<Value, Error> {
    use crate::repos;
    match request {
        Request::Cancel => {
            CANCEL.store(true, Relaxed);
            signin::cancel();
            Ok(Value::Null)
        }
        Request::AddRepo { url, trust } => repos::add(&url, trust.as_deref()),
        Request::ServerRepos { url, trust } => repos::server::listed(&url, trust.as_deref()),
        Request::Accounts => Ok(using::list()),
        Request::UseAccount { url, account } => using::use_for(&url, &account),
        Request::UnlinkAccount { url } => using::unlink(&url),
        Request::RemoveAccount { account } => using::remove(&account),
        Request::Repos => repos::list(),
        Request::RemoveRepo { repo_id } => repos::remove(&repo_id),
        Request::Browse { query } => repos::browse(&query),
        Request::Install { repo_id, id } => repos::install(&repo_id, &id),
        Request::Thumb { spec } => repos::preview::thumb(&spec),
        Request::InstallBase { uid } => crate::kin::install_base(&uid),
        Request::Votes { ids } => crate::votes::counts(&ids),
        Request::Vote { id, vote } => crate::votes::cast(&id, &vote),
        Request::SignIn { url } => waiting(|rt| rt.block_on(signin::start(&url))),
        Request::SignInWait => waiting(|rt| rt.block_on(signin::wait())),
    }
}

fn waiting(go: impl FnOnce(&tokio::runtime::Runtime) -> Result<Value, Error>) -> Result<Value, Error> {
    let _busy = BUSY.try_lock().map_err(|_| {
        said("android.another_account_task", "another account task is running; try again when it ends", &[])
    })?;
    CANCEL.store(false, Relaxed);
    go(&tokio::runtime::Builder::new_current_thread().enable_all().build()?)
}
