use crate::App;
use crate::auth::{Auth, Role};
use crate::db::one;
use crate::error::Res;
use crate::frame::Frame;
use crate::lang::{t, tf};
use crate::layout::setup_screen;
use crate::ui::{Kind, Row, Tone, copyable, link_button, rows, status};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use maud::html;

pub async fn screen(app: &App, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.at_least(Role::Admin).is_ok()) else {
        return Ok(Redirect::to("/").into_response());
    };
    let count = app.db.run(|c| one::<i64>(c, "SELECT COUNT(*) FROM packages", [])).await?.unwrap_or_default();
    let settings = app.settings();
    let repo = app
        .repos
        .first()
        .info(&app.cfg)
        .and_then(|v| v["url"].as_str().map(str::to_owned))
        .unwrap_or_default();
    let name = settings.repo_name.clone().unwrap_or_else(|| t("Your wallpaper server").into());
    let line = html! {
        p.status-line {
            (status(Tone::Good, t("Set up")))
            @if count == 0 { span.faint { (t("No wallpapers yet")) } }
        }
    };
    let web = Row::new(html! { (t("The web")) }).value(html! { span.fact { (app.cfg.public_url) } });
    let body = html! {
        div.empty {
            b { (t("Done: your server already hands out wallpapers.")) }
            p { (t("What you publish appears in the app of whoever adds this repository.")) }
        }
        section.section aria-labelledby="app" {
            h2 #app { (t("For the app")) }
            p.text { (t("In the app, paste the repository address below in Explore → Add a repository.")) }
            (rows(vec![web], true))
            h3.subtitle { (t("Repository address")) }
            (copyable(&repo, t("Copy"), t("Copied")))
        }
        section.section aria-labelledby="upload" {
            h2 #upload { (t("To upload wallpapers")) }
            ol.text.step-list {
                li { (t("In the desktop app, add the repository above from Explore.")) }
                li { (tf("In the editor of your wallpaper, press {} and choose the repository; sign in as {} when it asks.", &[&t("Publish…"), &auth.handle])) }
            }
            p.text { (t("Without the app, you can also upload a .wpak file from your Account page in this browser.")) }
        }
        div.actions.final {
            (link_button(Kind::Primary, "/admin", t("Go to Administration")))
            span.why { (t("There you see the health of the server; the invitations are in People.")) }
        }
    };
    let page = setup_screen(app, Frame::new(&name).line(line), body);
    Ok(([(header::CACHE_CONTROL, "no-store")], page).into_response())
}
