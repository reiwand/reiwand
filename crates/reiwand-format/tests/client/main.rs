#[path = "../fixture/mod.rs"]
mod fixture;

use fixture::{DAWN, DAY, DUSK, NOW, Repo, file_url, files, key};
use reiwand_format::{
    Anchor, Error, FileFetch, Reason, add, add_repo, browse, install, installed, publish, remove,
    remove_repo, repos, revoke, revoke_key, update_repo, verify,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;

mod keys;
mod trust;

fn mode(path: &std::path::Path) -> u32 {
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}
