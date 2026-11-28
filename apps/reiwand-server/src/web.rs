use crate::auth::Auth;
use crate::catalog::{SORTS, Search, query, search};
use crate::db::one;
use crate::error::{Problem, Res};
use crate::lang::{t, tf, word};
pub use crate::layout::page;
use crate::layout::preview;
use crate::ui::{Kind, button, checkbox, empty};
use crate::{Router, St};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use maud::{Markup, html};
use serde_json::Value;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(crate::explore::list))
        .route("/r/{repo}/", get(repo_catalog))
        .route("/r/{repo}/packages/{id}", get(repo_package))
        .route("/media/{file}", get(media))
}

pub fn is_page(path: &str) -> bool {
    let rest = path.strip_prefix("/r/").unwrap_or_default();
    matches!(rest.split('/').collect::<Vec<_>>()[..], [_, ""] | [_, "packages", _])
}

pub fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}

#[derive(serde::Deserialize)]
struct Ask {
    #[serde(default)]
    ask: String,
}

const FACETS: [&str; 8] =
    ["audio", "pointer", "sound", "customizable", "localized", "photosensitive", "mature", "4k"];

async fn repo_catalog(
    State(app): St,
    auth: Option<Auth>,
    Path(name): Path<String>,
    Query(s): Query<Search>,
) -> Res<Markup> {
    let name = crate::scope::resolve(&app, &name)?.name.clone();
    catalog(app, auth, name, s).await
}

async fn repo_package(
    State(app): St,
    auth: Option<Auth>,
    Path((name, id)): Path<(String, String)>,
    Query(a): Query<Ask>,
) -> Res<Markup> {
    let name = crate::scope::resolve(&app, &name)?.name.clone();
    crate::package_page::package(app, auth, name, id, a.ask).await
}

async fn catalog(
    app: std::sync::Arc<crate::App>,
    auth: Option<Auth>,
    repo: String,
    s: Search,
) -> Res<Markup> {
    crate::access::read(&app, &repo, auth.as_ref()).await?;
    let base = format!("/r/{repo}");
    let named = repo.clone();
    let (found, s) = app.db.run(move |c| Ok((search(c, &named, &s)?, s))).await?;
    let (q, kind, rating, facet) = (&s.q, &s.kind, &s.content, &s.facet);
    let (picked, sort) = (!s.featured.is_empty(), if s.sort.is_empty() { "newest" } else { &s.sort });
    let link = |n: u32| match query(&s, n) {
        rest if rest.is_empty() => format!("{base}/"),
        rest => format!("{base}/?{rest}"),
    };
    let body = html! {
        form role="search" method="get" action={ (base) "/" } {
            label { (t("Search")) input type="search" name="q" value=(q); }
            label { (t("Type")) select name="kind" {
                option value="" { (t("Any")) }
                @for k in ["image", "video", "scene", "web"] { option value=(k) selected[kind == k] { (word(k)) } }
            } }
            label { (t("Rating")) select name="content" {
                option value="" { (t("Any")) }
                @for r in ["general", "mature", "unrated"] { option value=(r) selected[rating == r] { (word(r)) } }
            } }
            label { (t("With")) select name="facet" {
                option value="" { (t("Anything")) }
                @for f in FACETS { option value=(f) selected[facet == f] { (word(f)) } }
            } }
            label { (t("Sort by")) select name="sort" {
                @for o in SORTS { option value=(o) selected[sort == o] { (sorted(o)) } }
            } }
            (checkbox(html! { input type="checkbox" name="featured" value="true" checked[picked]; }, t("Featured only"), None))
            (button(Kind::Primary, t("Search")))
        }
        @if found.items.is_empty() { (empty(t("Nothing found"), html! { (t("Try other words or clear the filters.")) })) }
        ul.grid { @for i in &found.items { li {
            (preview(text(i, "thumbnail"), "", 320, i["content"] == "mature"))
            a href={ (base) "/packages/" (text(i, "id")) } { (text(i, "title")) }
            br;
            small {
                @if i["featured"] == true { (t("Featured")) " · " }
                (word(text(i, "kind"))) " · " (text(i, "license")) " · ▲ " (i["votes"]["up"]) " ▼ " (i["votes"]["down"])
                @if let Some(o) = i["owner"].as_str() { " · " (o) }
            }
        } } }
        @if found.pages > 1 {
            nav.pages aria-label=(t("Pages")) {
                @if found.page > 1 { a.button rel="prev" href=(link(found.page - 1)) { (t("Previous")) } }
                span aria-current="page" { (tf("Page {} of {}", &[&found.page, &found.pages])) }
                @if found.page < found.pages { a.button rel="next" href=(link(found.page + 1)) { (t("Next")) } }
            }
        }
    };
    Ok(page(&app, auth.as_ref(), "/", &tf("Wallpapers in {}", &[&repo]), body))
}

fn sorted(sort: &str) -> &'static str {
    match sort {
        "title" => t("Title"),
        "votes" => t("Most votes"),
        _ => t("Newest"),
    }
}

async fn media(State(app): St, Path(file): Path<String>) -> Res<Response> {
    let lower =
        |s: &&str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    let sha = file.strip_suffix(".jpg").filter(lower).ok_or_else(|| Problem::missing("image"))?.to_string();
    let (key, sql) =
        (sha.clone(), "SELECT 1 FROM versions WHERE sha256 = ?1 AND state IN ('pending', 'published')");
    app.db.run(move |c| one::<i64>(c, sql, [key])).await?.ok_or_else(|| Problem::missing("image"))?;
    let path = app.cfg.data.join("thumbs").join(format!("{sha}.jpg"));
    let file = tokio::fs::File::open(path).await.map_err(|_| Problem::missing("image"))?;
    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
    ];
    Ok((headers, Body::from_stream(tokio_util::io::ReaderStream::new(file))).into_response())
}
