use super::{block_on, random, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed, invalid};
use crate::jobs::Ctl;
use reiwand_account::{Api, hex};
use reiwand_core::model::{JobKind, UploadOutput};
use reqwest::Method;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub(super) struct Destination {
    pub api: Api,
    pub name: String,
    pub repo_id: Option<String>,
}

impl Daemon {
    pub(super) fn destination(&self) -> Result<Destination> {
        let accounts = self.account_list();
        let found = accounts.iter().filter(|r| !r.pending).find_map(|r| {
            let link = r.links.iter().find(|l| reiwand_format::server_repo_name(&l.url).is_some())?;
            Some((r, link))
        });
        let Some((rec, link)) = found else {
            return Err(failed("sign in to an account linked to a repository first"));
        };
        let token =
            self.account_token(&rec.id).ok_or_else(|| failed("the access of this account is missing"))?;
        let pin = self.repo_pins().get(&rec.origin).cloned();
        let api = Api::pinned(&rec.origin, pin.as_deref()).map_err(sync_error)?.with_token(&token);
        let name = reiwand_format::server_repo_name(&link.url).unwrap_or_default().to_owned();
        Ok(Destination { api, name, repo_id: link.repo_id.clone() })
    }

    pub fn upload(self: &Arc<Self>, path: &str, visibility: &str) -> Result<u64> {
        if !matches!(visibility, "public" | "unlisted") {
            return Err(invalid("the visibility is public or unlisted"));
        }
        let (path, visibility) = (PathBuf::from(path), visibility.to_owned());
        if !path.is_absolute() {
            return Err(invalid("give the absolute path of a .wpak package"));
        }
        let size =
            std::fs::metadata(&path).map_err(|e| invalid(format!("cannot read the package: {e}")))?.len();
        self.destination()?;
        let target = hex(&random::<8>());
        Ok(self
            .start_job(JobKind::Upload, &target, move |d, ctl| d.send_package(&path, size, &visibility, ctl)))
    }

    pub fn own_packages(&self) -> Result<String> {
        let found = block_on(self.destination()?.api.get("/api/v1/me/packages")).map_err(sync_error)?;
        Ok(json!({"items": found["items"]}).to_string())
    }

    pub fn set_visibility(&self, id: &str, version: &str, visibility: &str) -> Result<()> {
        if !reiwand_format::valid_id(id) || !matches!(visibility, "public" | "unlisted") {
            return Err(invalid("give a package id and public or unlisted"));
        }
        let to = self.destination()?;
        let version = Some(version).filter(|v| !v.is_empty());
        let body = json!({"visibility": visibility, "version": version});
        let path = format!("/api/v1/r/{}/me/packages/{id}/visibility", to.name);
        block_on(to.api.post(&path, body)).map_err(sync_error)?;
        Ok(())
    }

    fn send_package(&self, path: &Path, size: u64, visibility: &str, ctl: &Ctl) -> Result<Value> {
        let to = self.destination()?;
        let api = to.api;
        let sha256 = reiwand_format::sha256_file(path)?;
        let asked = json!({"size": size, "sha256": sha256, "visibility": visibility, "repo": to.name});
        let opened = block_on(api.post("/api/v1/uploads", asked)).map_err(sync_error)?;
        let id = opened["id"].as_str().unwrap_or_default().to_owned();
        if visibility != "public" && opened["visibility"] != visibility {
            drop(block_on(api.slow(Method::DELETE, &format!("/api/v1/uploads/{id}"))));
            return Err(failed("this server does not offer unlisted uploads; nothing was uploaded"));
        }
        ctl.check()?;
        block_on(api.put_file(&format!("/api/v1/uploads/{id}"), path, size)).map_err(sync_error)?;
        ctl.check()?;
        let status = block_on(api.slow(Method::POST, &format!("/api/v1/uploads/{id}/finalize")))
            .map_err(sync_error)?;
        let text = |field: &str| status[field].as_str().map(String::from);
        let errors: Vec<String> = status["errors"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|e| e.as_str())
            .map(String::from)
            .collect();
        let state = text("state").unwrap_or_default();
        if matches!(state.as_str(), "failed" | "rejected") {
            return Err(failed(format!("the server refused the package: {}", errors.join("; "))));
        }
        let output = UploadOutput { id, state, package: text("package"), version: text("version"), errors };
        Ok(serde_json::to_value(output).unwrap_or_default())
    }
}
