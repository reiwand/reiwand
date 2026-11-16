use crate::St;
use crate::auth::Auth;
use crate::error::{Problem, Res};
use axum::body::Body;
use axum::extract::{Path as Route, State};
use axum::http::{StatusCode, header};
use axum::response::Response;
use rusqlite::params;
use std::collections::HashMap;
use std::io::ErrorKind;
use tokio_util::io::ReaderStream;

pub async fn serve(
    State(app): St,
    auth: Option<Auth>,
    Route(mut found): Route<HashMap<String, String>>,
) -> Res<Response> {
    let path = found.remove("path").unwrap_or_default();
    let repo = crate::scope::resolve(&app, found.get("repo").map_or("", String::as_str))?;
    crate::access::read(&app, &repo.name, auth.as_ref()).await?;
    let private = crate::access::is_private(&app, &repo.name).await;
    let parts: Vec<&str> = path.split('/').collect();
    let public = matches!(
        parts[0],
        "root" | "entry" | "index" | "revoked" | "authors" | "featured" | "previews" | "packages"
    );
    let clean = |s: &&str| {
        !s.is_empty()
            && !s.starts_with('.')
            && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-+".contains(&b))
    };
    if !public || !parts.iter().all(clean) {
        return Err(Problem::missing("file"));
    }
    if let ["packages", id, file] = parts[..] {
        let (id, version, name) =
            (id.to_string(), file.trim_end_matches(".wpak").to_string(), repo.name.clone());
        let sql = "SELECT EXISTS (SELECT 1 FROM versions WHERE repo = ?3 AND package_id = ?1 AND version = ?2 \
                   AND state IN ('removed', 'revoked'))";
        if app
            .db
            .run(move |c| Ok(c.query_row(sql, params![id, version, name], |r| r.get::<_, bool>(0))?))
            .await?
        {
            return Err(Problem::new(StatusCode::GONE, "withdrawn", "this package version was withdrawn"));
        }
    }
    let file = match tokio::fs::File::open(repo.dir.join(&path)).await {
        Err(e) if e.kind() == ErrorKind::NotFound => return Err(Problem::missing("file")),
        file => file?,
    };
    let meta = file.metadata().await?;
    if !meta.is_file() {
        return Err(Problem::missing("file"));
    }
    let kind = match path.rsplit('.').next().unwrap_or_default() {
        "json" => "application/json",
        "wpak" => "application/octet-stream",
        "webp" | "png" | "gif" | "avif" | "bmp" => {
            &format!("image/{}", path.rsplit('.').next().unwrap_or_default())
        }
        "jpg" | "jpeg" => "image/jpeg",
        "minisig" | "latest" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    };
    let cache = if private {
        "private, no-cache"
    } else if path == "entry/latest" {
        "no-cache"
    } else {
        "public, max-age=31536000, immutable"
    };
    let mut response =
        Response::builder().header(header::CONTENT_TYPE, kind).header(header::CACHE_CONTROL, cache);
    response = response.header(header::CONTENT_LENGTH, meta.len());
    if path.ends_with(".wpak") {
        response = response.header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", parts[1..].join("-")),
        );
    }
    response.body(Body::from_stream(ReaderStream::new(file))).map_err(Problem::internal)
}
