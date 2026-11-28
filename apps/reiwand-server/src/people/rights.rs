use super::view::{Account, People};
use super::{admin, login};
use crate::auth::Auth;
use crate::error::{Problem, Res};
use crate::lang::t;
use crate::ui::{Kind, Row, button, fold, rows};
use crate::{App, St};
use axum::Form;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use maud::{Markup, html};
use std::collections::HashMap;

pub struct Grant {
    pub user: i64,
    pub repo: String,
    pub publish: bool,
}

pub struct Repository {
    pub name: String,
    pub private: bool,
}

pub async fn load(app: &App) -> Res<(Vec<Repository>, Vec<Grant>)> {
    app.db
        .run(|c| {
            let mut q = c.prepare("SELECT name, private FROM repos ORDER BY n")?;
            let repos = q.query_map([], |r| Ok(Repository { name: r.get(0)?, private: r.get(1)? }))?;
            let repos = repos.collect::<Result<Vec<_>, _>>()?;
            let mut q =
                c.prepare("SELECT user_id, repo, may_publish FROM repo_members WHERE may_read = 1")?;
            let grants =
                q.query_map([], |r| Ok(Grant { user: r.get(0)?, repo: r.get(1)?, publish: r.get(2)? }))?;
            Ok((repos, grants.collect::<Result<Vec<_>, _>>()?))
        })
        .await
}

fn level(p: &People, a: &Account, repo: &str) -> &'static str {
    match p.grants.iter().find(|g| g.user == a.id && g.repo == repo) {
        Some(g) if g.publish => "publish",
        Some(_) => "read",
        None => "none",
    }
}

pub fn section(p: &People) -> Markup {
    let items = p.accounts.iter().map(|a| {
        let form = html! {
            form.fields-line method="post" action={ "/admin/people/accounts/" (a.id) "/rights" } {
                @for r in &p.repos {
                    @let now = level(p, a, &r.name);
                    div.field {
                        label for={ "g" (a.id) "-" (r.name) } { (r.name) @if r.private { " (" (t("private")) ")" } }
                        select id={ "g" (a.id) "-" (r.name) } name={ "repo_" (r.name) } {
                            option value="none" selected[now == "none"] { (t("No rights")) }
                            option value="read" selected[now == "read"] { (t("Can see and download")) }
                            option value="publish" selected[now == "publish"] { (t("Can also publish")) }
                        }
                    }
                }
                div.field { (button(Kind::Plain, t("Save"))) }
            }
        };
        Row::new(html! { (a.handle) }).more(fold(t("Repository rights"), form, false))
    });
    html! {
        section.section #rights aria-labelledby="rights-title" {
            h2 #rights-title { (t("Rights in repositories")) }
            p.intro { (t("Whoever has no rights still sees the public repositories. Administrators and moderators see every repository.")) }
            (rows(items.collect(), false))
        }
    }
}

pub async fn change(
    State(app): St,
    auth: Option<Auth>,
    Path(id): Path<i64>,
    Form(f): Form<HashMap<String, String>>,
) -> Res<Response> {
    let Some(auth) = admin(auth) else { return Ok(login()) };
    let mut set = Vec::new();
    for (key, value) in &f {
        let Some(repo) = key.strip_prefix("repo_") else { continue };
        let (read, publish) = match value.as_str() {
            "none" => (false, false),
            "read" => (true, false),
            "publish" => (true, true),
            _ => return Err(Problem::bad("rights: none, read or publish")),
        };
        set.push((repo.to_string(), read, publish));
    }
    app.db
        .run(move |c| {
            for (repo, read, publish) in &set {
                let known = c.query_row("SELECT COUNT(*) FROM repos WHERE name = ?1", [repo], |r| {
                    r.get::<_, i64>(0)
                })?;
                crate::db::changed(known as usize, "repository")?;
                crate::members::grant(c, repo, id, *read, *publish)?;
            }
            let detail = format!("{} repositories", set.len());
            crate::db::audit(c, Some(auth.user), "repo-rights", &id.to_string(), &detail)?;
            Ok(())
        })
        .await?;
    Ok(Redirect::to("/admin/people#rights").into_response())
}
