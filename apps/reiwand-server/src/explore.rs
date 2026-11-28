use crate::access::visible;
use crate::auth::Auth;
use crate::db::rows;
use crate::error::Res;
use crate::lang::{t, tf};
use crate::layout::page;
use crate::ui::{Kind, copyable_with, empty, link_button};
use crate::units::date;
use crate::{App, St};
use axum::extract::State;
use maud::{Markup, html};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::Value;

const STATS: &str = "SELECT v.repo AS repo, COUNT(DISTINCT v.package_id) AS wallpapers, \
                     MAX(v.published_at) AS updated FROM versions v \
                     JOIN packages p ON p.repo = v.repo AND p.id = v.package_id \
                     WHERE v.state = 'published' AND v.listed = 1 AND p.hidden IS NULL GROUP BY v.repo";

struct Entry {
    name: String,
    address: String,
    private: bool,
    wallpapers: i64,
    updated: Option<i64>,
}

fn open_link(address: &str) -> String {
    format!("reiwand://add?url={}", utf8_percent_encode(address, NON_ALPHANUMERIC))
}

async fn entries(app: &App, auth: Option<&Auth>) -> Res<Vec<Entry>> {
    let names: Vec<String> = app.repos.ready().iter().map(|r| r.name.clone()).collect();
    let seen = visible(app, auth, names).await;
    let stats: Vec<Value> = app.db.run(|c| rows(c, STATS, rusqlite::params![])).await?;
    let stat = |name: &str| stats.iter().find(|s| s["repo"] == name);
    Ok(seen
        .into_iter()
        .filter(|(_, rights)| rights.may_read)
        .map(|(name, rights)| {
            let found = stat(&name);
            Entry {
                address: app.cfg.url(&format!("/r/{name}")),
                private: rights.private,
                wallpapers: found.and_then(|s| s["wallpapers"].as_i64()).unwrap_or(0),
                updated: found.and_then(|s| s["updated"].as_i64()),
                name,
            }
        })
        .collect())
}

fn card(e: &Entry) -> Markup {
    html! {
        section.section {
            h2 { code { (e.name) } @if e.private { " " small { (t("Private")) } } }
            p.text {
                @if e.wallpapers == 1 { (t("1 wallpaper")) } @else { (tf("{} wallpapers", &[&e.wallpapers])) } " · "
                @if let Some(at) = e.updated { (tf("Last update {}", &[&date(at)])) } @else { (t("No updates yet")) }
            }
            (copyable_with(&e.address, t("Copy the address"), t("Address copied"), Kind::Plain))
            p {
                (link_button(Kind::Primary, &open_link(&e.address), t("Open in Reiwand")))
                " " (link_button(Kind::Plain, &format!("/r/{}/", e.name), t("Browse the catalog")))
            }
        }
    }
}

pub async fn list(State(app): St, auth: Option<Auth>) -> Res<Markup> {
    let items = entries(&app, auth.as_ref()).await?;
    let body = html! {
        p.text { (t("Wallpaper repositories hosted here. Copy the address into Reiwand or open it directly.")) }
        @if items.is_empty() {
            (empty(t("No repositories to show"), html! { (t("This server has no public repositories. If you are a member of a private one, sign in.")) }))
        }
        @for e in &items { (card(e)) }
    };
    Ok(page(&app, auth.as_ref(), "/", t("Explore repositories"), body))
}
