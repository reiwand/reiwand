use crate::admin_repos::infos;
use crate::auth::{Auth, Role};
use crate::error::{Problem, Res};
use crate::lang::t;
use crate::lang::tf;
use crate::ui::{Kind, Tone, button_named, confirm, link_button, notice};
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
    #[serde(default)]
    delete: String,
}

pub async fn screen(State(app): St, auth: Option<Auth>, Query(q): Query<Shown>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.role >= Role::Admin) else {
        return Ok(Redirect::to("/login?next=/admin/repos").into_response());
    };
    Ok(draw(&app, &auth, &q, None, "").await?.into_response())
}

pub async fn failed(app: &Arc<App>, auth: &Auth, typed: &str, problem: Problem) -> Res<Response> {
    let mut response =
        draw(app, auth, &Shown::default(), Some(&problem.detail), typed).await?.into_response();
    *response.status_mut() = problem.status;
    Ok(response)
}

async fn draw(app: &Arc<App>, auth: &Auth, q: &Shown, error: Option<&str>, typed: &str) -> Res<Markup> {
    let new = q.new.as_str();
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
        p.text { (t("Each repository has its own address and its own keys. Moderation and accounts are shared by all of them.")) }
        p.text { (t("A private repository shows only to the accounts you give read rights in the People page; publishing there needs publish rights.")) }
        @for i in &items {
            @let name = text(i, "name");
            @let (private, mode) = state.iter().find(|s| s.0 == name).map_or((false, "open"), |s| (s.1, s.2.as_str()));
            section.section.new[name == new] id=(name) {
                h2 { code { (name) } }
                (crate::ui::copyable(text(i, "url"), t("Copy"), t("Copied")))
                @if i["root_key_kept"] == false {
                    p.text { (t("This repository has no backup root key: it was created before the server kept it.")) }
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
                (ending(name, q.delete == name))
            }
        }
    };
    Ok(page(app, Some(auth), "/admin/repos", t("Repositories"), body))
}

fn ending(name: &str, asking: bool) -> Markup {
    if !asking {
        let href = format!("/admin/repos?delete={name}#{name}");
        return html! { div.actions { (link_button(Kind::Danger, &href, t("Delete this repository…"))) } };
    }
    let ask = tf(
        "Delete the repository \"{}\" for good? Its address stops answering, and its packages, votes, members, keys and signed index are deleted. Devices that added it keep what they installed but get nothing new from it; mirrors and backups you made keep their own copy. This cannot be undone.",
        &[&name],
    );
    let buttons = html! {
        (button_named(Kind::Danger, "confirm", "true", t("Delete")))
        (link_button(Kind::Plain, &format!("/admin/repos#{name}"), t("Cancel")))
    };
    let action = format!("/api/v1/admin/repos/{name}?_method=DELETE&back=/admin/repos");
    confirm(&ask, t("Confirm"), &action, buttons)
}
