use crate::db::rows;
use crate::error::{Problem, Res};
use crate::scope::{Scope, at};
use crate::web::text;
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use rusqlite::{Connection, params};
use serde::Deserialize;
use serde_json::{Value, json};

const SELECT: &str = "SELECT v.package_id AS id, v.version, v.kind, v.title, v.description, \
                      v.license, v.tags, v.authors, v.size, v.sha256, v.published_at, u.handle AS owner, \
                      v.content, v.photosensitive, v.facets, p.hidden, v.listed, v.repo, \
                      (SELECT COALESCE(SUM(value = 1), 0) FROM votes WHERE repo = v.repo AND package_id = v.package_id) AS up, \
                      (SELECT COALESCE(SUM(value = -1), 0) FROM votes WHERE repo = v.repo AND package_id = v.package_id) AS down, \
                      EXISTS (SELECT 1 FROM featured f WHERE f.repo = v.repo AND f.package_id = v.package_id) AS featured \
                      FROM versions v JOIN packages p ON p.repo = v.repo AND p.id = v.package_id \
                      LEFT JOIN users u ON u.id = p.owner_id";
const FROM: &str = " FROM versions v JOIN packages p ON p.repo = v.repo AND p.id = v.package_id";
const NEWEST: &str = " WHERE v.repo = ?7 AND v.state = 'published' AND p.hidden IS NULL AND v.id = (SELECT w.id FROM versions w \
                      WHERE w.repo = v.repo AND w.package_id = v.package_id AND w.state = 'published' AND w.listed = 1 \
                      ORDER BY w.published_at DESC, w.id DESC LIMIT 1) \
                      AND (?1 = '' OR (?1 = 'm' AND v.id IN (SELECT rowid FROM search WHERE search MATCH ?2)) \
                      OR (?1 = 't' AND v.tags LIKE ?2 ESCAPE '!')) \
                      AND (?3 = '' OR v.kind = ?3) \
                      AND (?4 = '' OR v.content = ?4 OR (?4 = 'unrated' AND v.content IS NULL)) \
                      AND (?5 = '' OR EXISTS (SELECT 1 FROM json_each(v.facets) WHERE value = ?5)) \
                      AND (?6 = '' OR EXISTS (SELECT 1 FROM featured f WHERE f.repo = v.repo AND f.package_id = v.package_id))";
pub const SORTS: [&str; 3] = ["newest", "title", "votes"];

pub fn routes() -> Router {
    let router = at(Router::new(), "/api/v1/catalog", get(list));
    let router = at(router, "/api/v1/packages/{id}", get(package));
    at(router, "/api/v1/packages/{id}/versions", get(versions))
}

#[derive(Deserialize, Default)]
pub struct Search {
    #[serde(default)]
    pub q: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub sort: String,
    pub page: Option<u32>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub facet: String,
    #[serde(default)]
    pub featured: String,
    pub limit: Option<u32>,
}

fn shape(mut item: Value) -> Value {
    for key in ["tags", "authors", "facets"] {
        item[key] = serde_json::from_str(item[key].as_str().unwrap_or("[]")).unwrap_or_default();
    }
    item["photosensitive"] = item["photosensitive"].as_i64().map(|f| f != 0).into();
    item["listed"] = (item["listed"] != 0).into();
    item["featured"] = (item["featured"] != 0).into();
    item["votes"] = json!({"up": item["up"], "down": item["down"]});
    let (id, version, sha) = (text(&item, "id"), text(&item, "version"), text(&item, "sha256"));
    let (thumbnail, download) =
        (format!("/media/{sha}.jpg"), format!("/r/{}/packages/{id}/{version}.wpak", text(&item, "repo")));
    (item["thumbnail"], item["download"]) = (thumbnail.into(), download.into());
    item.as_object_mut()
        .into_iter()
        .for_each(|fields| ["up", "down"].iter().for_each(|k| drop(fields.remove(*k))));
    item
}

fn pattern(q: &str) -> (&'static str, String) {
    let long: Vec<String> = q
        .split_whitespace()
        .filter(|term| term.chars().count() >= 3)
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect();
    match q.trim() {
        "" => ("", String::new()),
        _ if !long.is_empty() => ("m", long.join(" ")),
        short => ("t", format!("%\"{}%", short.replace('!', "!!").replace('%', "!%").replace('_', "!_"))),
    }
}

pub struct Found {
    pub items: Vec<Value>,
    pub page: u32,
    pub pages: u32,
    pub total: u32,
}

fn order(sort: &str) -> Res<&'static str> {
    match sort {
        "" | "newest" => Ok("v.published_at DESC, v.id DESC"),
        "title" => Ok("v.title COLLATE NOCASE, v.package_id"),
        "votes" => Ok("up - down DESC, up DESC, v.published_at DESC, v.id DESC"),
        _ => Err(Problem::bad("sort: newest, title or votes")),
    }
}

pub fn search(c: &Connection, repo: &str, s: &Search) -> Res<Found> {
    let limit = s.limit.unwrap_or(24).clamp(1, 100);
    let page = s.page.unwrap_or(1).max(1);
    let order = order(&s.sort)?;
    let (mode, pattern) = pattern(&s.q);
    let featured = if s.featured.is_empty() || s.featured == "false" { "" } else { "1" };
    let filters = params![mode, pattern, s.kind, s.content, s.facet, featured, repo];
    let total: u32 = c.query_row(&format!("SELECT COUNT(*){FROM}{NEWEST}"), filters, |r| r.get(0))?;
    let sql = format!("{SELECT}{NEWEST} ORDER BY {order} LIMIT ?8 OFFSET ?9");
    let skip = i64::from(page - 1) * i64::from(limit);
    let values = params![mode, pattern, s.kind, s.content, s.facet, featured, repo, limit, skip];
    let items = rows(c, &sql, values)?.into_iter().map(shape).collect();
    Ok(Found { items, page, pages: total.div_ceil(limit).max(1), total })
}

pub fn details(c: &Connection, repo: &str, id: &str) -> Res<(Value, Vec<Value>)> {
    let sql = format!(
        "{SELECT} WHERE v.repo = ?2 AND v.package_id = ?1 AND v.state = 'published' ORDER BY v.published_at DESC, v.id DESC"
    );
    let versions: Vec<Value> = rows(c, &sql, [id, repo])?.into_iter().map(shape).collect();
    let latest = versions.first().cloned().ok_or_else(|| Problem::missing("package"))?;
    Ok((latest, versions))
}

pub fn query(s: &Search, page: u32) -> String {
    let encode = |text: &str| utf8_percent_encode(text, NON_ALPHANUMERIC).to_string();
    let limit = s.limit.map(|l| l.to_string()).unwrap_or_default();
    let filters = [
        ("q", encode(&s.q)),
        ("kind", encode(&s.kind)),
        ("content", encode(&s.content)),
        ("facet", encode(&s.facet)),
        ("featured", encode(&s.featured)),
        ("sort", encode(&s.sort)),
        ("limit", limit),
    ];
    let kept = filters.iter().filter(|(_, v)| !v.is_empty()).map(|(k, v)| format!("{k}={v}"));
    let page = (page > 1).then(|| format!("page={page}"));
    kept.chain(page).collect::<Vec<_>>().join("&")
}

async fn list(State(app): St, sc: Scope, Query(s): Query<Search>) -> Res<impl IntoResponse> {
    let repo = sc.repo.clone();
    let (found, s) = app.db.run(move |c| Ok((search(c, &sc.repo, &s)?, s))).await?;
    let near = [(found.page + 1, "next"), (found.page.wrapping_sub(1), "prev")];
    let links: Vec<String> = near
        .iter()
        .filter(|(n, _)| (1..=found.pages).contains(n))
        .map(|(n, rel)| format!("</api/v1/r/{repo}/catalog?{}>; rel=\"{rel}\"", query(&s, *n)))
        .collect();
    let link = (!links.is_empty()).then(|| [(header::LINK, links.join(", "))]);
    let answer =
        json!({"items": found.items, "page": found.page, "pages": found.pages, "total": found.total});
    Ok((link, Json(answer)))
}

async fn package(State(app): St, sc: Scope) -> Reply {
    let (mut latest, versions) = app.db.run(move |c| details(c, &sc.repo, &sc.id)).await?;
    latest["versions"] = versions.iter().map(|v| v["version"].clone()).collect();
    Ok(Json(latest))
}

async fn versions(State(app): St, sc: Scope) -> Reply {
    let (_, versions) = app.db.run(move |c| details(c, &sc.repo, &sc.id)).await?;
    Ok(Json(json!({"items": versions})))
}
