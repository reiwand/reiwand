use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use serde_json::{Map, json};

const ELSEWHERE: &str = "votes go only to packages of a repository you have an account for";

impl Daemon {
    pub fn vote(&self, spec: &str, vote: &str) -> Result<String> {
        let specs = [spec.to_owned()];
        let repo = spec.split_once('/').map_or("", |(repo, _)| repo);
        let first = || self.destinations().into_iter().next().ok_or_else(|| invalid(ELSEWHERE));
        let to = if repo.is_empty() {
            first()?
        } else {
            self.destination_for(repo).map_err(|_| invalid(ELSEWHERE))?
        };
        let found = reiwand_format::voted(&specs, to.repo_id.as_deref());
        let Some(asked) = found.filter(|_| matches!(vote, "up" | "down" | "none")) else {
            return Err(invalid("give a package id or REPO/ID and up, down or none"));
        };
        let Some((_, id)) = asked.first() else { return Err(invalid(ELSEWHERE)) };
        let body = json!({ "vote": vote });
        let found = block_on(to.api.post(&format!("/api/v1/r/{}/packages/{id}/vote", to.name), body))
            .map_err(sync_error)?;
        Ok(found.to_string())
    }

    pub fn votes(&self, specs: &[String]) -> Result<String> {
        let ours = self.destinations();
        let repos: Vec<&str> = ours.iter().filter_map(|d| d.repo_id.as_deref()).collect();
        let none = json!({ "items": {}, "shown": false, "repos": repos }).to_string();
        if reiwand_format::voted(specs, None).is_none() {
            return Err(invalid("give at most 1000 package ids, each as ID or REPO/ID"));
        }
        if !self.state().config.repos.vote_counts || repos.is_empty() {
            return Ok(none);
        }
        let mut items = Map::new();
        for to in ours.iter().filter(|d| d.repo_id.is_some()) {
            let asked = reiwand_format::voted(specs, to.repo_id.as_deref()).unwrap_or_default();
            let mut found = Map::new();
            for query in reiwand_format::vote_queries(&to.name, &asked) {
                let page = block_on(to.api.get(&query)).map_err(sync_error)?;
                found.extend(page["items"].as_object().cloned().unwrap_or_default());
            }
            for (spec, id) in asked {
                if let Some(count) = found.get(id) {
                    items.insert(spec.into(), count.clone());
                }
            }
        }
        Ok(json!({ "items": items, "shown": true, "repos": repos }).to_string())
    }
}
