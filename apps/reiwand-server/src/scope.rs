use crate::App;
use crate::error::Problem;
use axum::extract::{FromRequestParts, RawPathParams};
use axum::http::request::Parts;
use std::sync::Arc;

pub struct Scope {
    pub repo: String,
    pub id: String,
}

impl FromRequestParts<Arc<App>> for Scope {
    type Rejection = Problem;

    async fn from_request_parts(parts: &mut Parts, app: &Arc<App>) -> Result<Scope, Problem> {
        let params = RawPathParams::from_request_parts(parts, app).await.map_err(Problem::internal)?;
        let get = |key: &str| params.iter().find(|(k, _)| *k == key).map(|(_, v)| v.to_string());
        let repo = match get("repo") {
            Some(name) => app.repos.get(&name).ok_or_else(|| Problem::missing("repository"))?.name.clone(),
            None => app.repos.first().name.clone(),
        };
        Ok(Scope { repo, id: get("id").unwrap_or_default() })
    }
}

pub fn first(app: &App) -> String {
    app.repos.first().name.clone()
}

pub fn resolve(app: &App, name: &str) -> Result<Arc<crate::repo::Repo>, Problem> {
    match name {
        "" => Ok(app.repos.first()),
        name => app.repos.get(name).ok_or_else(|| Problem::missing("repository")),
    }
}

pub fn both(
    router: crate::Router,
    path: &str,
    method: axum::routing::MethodRouter<Arc<App>>,
) -> crate::Router {
    let scoped = path.replacen("/api/v1/", "/api/v1/r/{repo}/", 1);
    router.route(path, method.clone()).route(&scoped, method)
}
