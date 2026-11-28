use crate::auth::Auth;
use crate::catalog::details;
use crate::error::Res;
use crate::lang::{t, tf, tm, word};
use crate::layout::preview;
use crate::ui::{Tone, notice};
use crate::web::{page, text};
use maud::{Markup, PreEscaped, html};
use reiwand_format::markup;
use serde_json::Value;

fn joined(value: &Value) -> String {
    value.as_array().into_iter().flatten().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")
}

pub async fn package(
    app: std::sync::Arc<crate::App>,
    auth: Option<Auth>,
    repo: String,
    id: String,
    ask: String,
) -> Res<Markup> {
    crate::access::read(&app, &repo, auth.as_ref()).await?;
    let (key, named, base) = (id.clone(), repo.clone(), format!("/r/{repo}"));
    let (latest, versions) = app.db.run(move |c| details(c, &named, &key)).await?;
    let (title, owner) = (text(&latest, "title"), latest["owner"].as_str().unwrap_or(t("a deleted account")));
    let render = if latest["content"].is_null() { markup::verbatim } else { markup::html };
    let description = render(text(&latest, "description"));
    let body = html! {
        (preview(text(&latest, "thumbnail"), &tf("Preview of {}", &[&title]), 640, latest["content"] == "mature"))
        @if let Some(reason) = latest["hidden"].as_str() {
            (notice(Tone::Warn, t("Out of the catalog"), html! { p { (tm("The moderators took this wallpaper out of the catalog: {}", &[html! { (reason) }])) } }, None))
        }
        @if versions.iter().any(|v| v["listed"] == false) {
            p.text { (t("Unlisted: this wallpaper, or some of its versions, stays out of the catalog and search; \
                whoever has this link can see and install it. It is not private: it stays in the signed \
                repository files, which anyone can read.")) }
        }
        @if latest["photosensitive"] == true {
            (notice(Tone::Warn, t("Flashing lights"), html! { p { (t("This wallpaper has flashing lights or patterns that can affect people with \
                photosensitive epilepsy.")) } }, None))
        }
        (PreEscaped(description))
        (crate::votes::ballot(&base, &id, &latest, auth.is_some()))
        dl {
            dt { (t("Package")) } dd { code { (id) } }
            dt { (t("Type")) } dd { (word(text(&latest, "kind"))) }
            dt { (t("License")) } dd { (text(&latest, "license")) }
            dt { (t("Authors")) } dd { (joined(&latest["authors"])) }
            dt { (t("Tags")) } dd { (joined(&latest["tags"])) }
            dt { (t("Rating")) } dd { (word(latest["content"].as_str().unwrap_or("unrated"))) }
            dt { (t("Facets")) } dd { (joined(&latest["facets"])) }
            dt { (t("Uploaded by")) } dd { (owner) }
            dt { "SHA-256" } dd { code { (text(&latest, "sha256")) } }
        }
        p {
            a href=(text(&latest, "download")) { (tf("Download version {}", &[&text(&latest, "version")])) }
            " · " (t("or install it with a client that verifies the signed repository."))
        }
        h2 { (t("Versions")) }
        ul { @for v in &versions { li { a href=(text(v, "download")) { (text(v, "version")) } } } }
        h2 { (t("Report this wallpaper")) }
        form.form method="post" action={ "/api/v1" (base) "/packages/" (id) "/reports?back=" (base) "/packages/" (id) } {
            label { (t("Reason")) select name="category" {
                @for c in ["copyright", "illegal", "malware", "mature", "spam", "other"] {
                    option value=(c) { (word(c)) }
                }
            } }
            label { (t("Details")) textarea name="details" rows="3" maxlength="4000" {} }
            button { (t("Send report")) }
        }
        p.text {
            (tm("For legal notices with your contact details use the {}.", &[html! { a href="/legal/notice" { (t("notice form")) } }]))
        }
        (crate::package_tools::tools(auth.as_ref(), &repo, &id, &latest, &ask))
    };
    Ok(page(&app, auth.as_ref(), "/", title, body))
}
