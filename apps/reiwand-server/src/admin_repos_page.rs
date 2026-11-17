use crate::admin_repos::infos;
use crate::auth::{Auth, Role};
use crate::error::{Problem, Res};
use crate::lang::t;
use crate::ui::{Tone, notice};
use crate::web::{page, text};
use crate::{App, St};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use maud::{Markup, html};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct Shown {
    #[serde(default)]
    new: String,
}

pub async fn screen(State(app): St, auth: Option<Auth>, Query(q): Query<Shown>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.role >= Role::Admin) else {
        return Ok(Redirect::to("/login?next=/admin/repos").into_response());
    };
    Ok(draw(&app, &auth, &q.new, None, "").await?.into_response())
}

pub async fn failed(app: &Arc<App>, auth: &Auth, typed: &str, problem: Problem) -> Res<Response> {
    let mut response = draw(app, auth, "", Some(&problem.detail), typed).await?.into_response();
    *response.status_mut() = problem.status;
    Ok(response)
}

async fn draw(app: &Arc<App>, auth: &Auth, new: &str, error: Option<&str>, typed: &str) -> Res<Markup> {
    let items = infos(app).await;
    let state: Vec<(String, bool, String)> = app
        .db
        .run(|c| {
            let mut q = c.prepare("SELECT name, private, publishing FROM repos")?;
            let found = q.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
            Ok(found.collect::<Result<Vec<_>, _>>()?)
        })
        .await?;
    let body = html! {
        @if let Some(error) = error {
            (notice(Tone::Bad, t("The repository was not created"), html! { p { (crate::lang::word(error)) } }, None))
        }
        @if items.iter().any(|i| text(i, "name") == new) {
            (notice(Tone::Good, t("Repository created"),
                html! { p { (t("Copy its address below to add it in the app, and choose who can see it and who can publish.")) } }, None))
        }
        form method="post" action="/admin/repos" {
            label { (t("Name")) input name="name" value=(typed) required maxlength="80"; }
            button { (t("Create the repository")) }
        }
        p.texto { (t("Each repository has its own address and its own keys. Moderation and accounts are shared by all of them.")) }
        p.texto { (t("A private repository shows only to the accounts you give read rights in the People page; publishing there needs publish rights.")) }
        @for i in &items {
            @let name = text(i, "name");
            @let (private, mode) = state.iter().find(|s| s.0 == name).map_or((false, "open"), |s| (s.1, s.2.as_str()));
            section.seccion.nueva[name == new] {
                h2 { code { (name) } }
                (crate::ui::copyable(text(i, "url"), t("Copy"), t("Copied")))
                @if i["root_key_kept"] == false {
                    p.texto { (t("This repository has no backup root key: it was created before the server kept it.")) }
                }
                form method={ "post" } action={ "/api/v1/admin/repos/" (name) "?_method=PATCH&back=/admin/repos" } {
                    label { (t("Who can see it"))
                        select name="private" {
                            option value="false" selected[!private] { (t("Anyone")) }
                            option value="true" selected[private] { (t("Only members")) }
                        }
                    }
                    label { (t("Who can publish"))
                        select name="publishing" {
                            option value="open" selected[mode == "open"] { (t("Any account, for review")) }
                            option value="members" selected[mode == "members"] { (t("Only members with publish rights")) }
                        }
                    }
                    button { (t("Save")) }
                }
            }
        }
    };
    Ok(page(app, Some(auth), "/admin/repos", t("Repositories"), body))
}
