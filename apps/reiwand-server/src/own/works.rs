use crate::lang::{t, tf, word};
use crate::ui::{Kind, Row, Tone, button, file_picker, fold, form_actions, rows, status, subheading};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

fn tone(state: &str) -> Tone {
    match state {
        "published" | "done" => Tone::Good,
        "rejected" | "removed" | "withdrawn" | "failed" => Tone::Bad,
        _ => Tone::Warn,
    }
}

fn state_text(state: &str) -> &str {
    match state {
        "pending" => t("Waiting for review"),
        other => word(other),
    }
}

fn change(v: &Value) -> Markup {
    let (id, version, now) = (text(v, "package"), text(v, "version"), text(v, "visibility"));
    let other = if now == "public" { "unlisted" } else { "public" };
    html! { form method="post" action={ "/api/v1/r/" (text(v, "repo")) "/me/packages/" (id) "/visibility?back=/account" } {
        input type="hidden" name="version" value=(version);
        input type="hidden" name="visibility" value=(other);
        button.button { (tf("Make {}", &[&word(other)])) }
    } }
}

fn version_row(v: &Value) -> Row {
    let state = text(v, "state");
    let value = html! {
        (status(tone(state), state_text(state)))
        span { (word(text(v, "visibility"))) }
        @if !text(v, "statement").is_empty() && tone(state) == Tone::Bad { span { (text(v, "statement")) } }
    };
    let row = Row::new(html! { (text(v, "title")) " " span.faint { (text(v, "version")) } }).value(value);
    match state {
        "rejected" | "removed" | "withdrawn" => row,
        _ => row.action(change(v)),
    }
}

fn upload_row(u: &Value) -> Row {
    let state = text(u, "state");
    let value = html! {
        (status(tone(state), word(state)))
        @if !text(u, "errors").is_empty() { span { (text(u, "errors")) } }
    };
    Row::new(html! { (tf("Upload {}", &[&text(u, "id")])) }).value(value)
}

pub fn upload_form(targets: &[String], lit: bool) -> Markup {
    html! {
        form.form method="post" action="/upload?back=/account" enctype="multipart/form-data" {
            (subheading(t("Upload a wallpaper")))
            div.fields-line {
                div.field {
                    label for="repository" { (t("Repository")) }
                    select #repository name="repo" {
                        @for name in targets { option value=(name) { (name) } }
                    }
                }
                div.field {
                    label for="visibility" { (t("Visibility")) }
                    select #visibility name="visibility" {
                        @for o in ["public", "unlisted"] { option value=(o) selected[o == "public"] { (word(o)) } }
                    }
                }
                div.field {
                    span.label #package-file { (t(".wpak file")) }
                    (file_picker(html! { input name="package" type="file" accept=".wpak" required aria-labelledby="package-file"; },
                        t("Choose the file…"), t("No file chosen")))
                }
            }
            (fold(t("What is \"unlisted\""), html! { p.text { (t("Unlisted versions stay out of the catalog and \
                search; whoever has the link to the package can still see and install them. Unlisted is not \
                private: they stay in the signed repository files, which anyone can read; keep private \
                wallpapers in sync or in a repository of your own.")) } }, false))
            (form_actions(button(if lit { Kind::Primary } else { Kind::Plain }, t("Upload"))))
        }
    }
}

pub fn works(versions: &[Value], uploads: &[Value], targets: &[String], lit: bool) -> Markup {
    let items: Vec<Row> = versions.iter().map(version_row).chain(uploads.iter().map(upload_row)).collect();
    html! {
        section.section aria-labelledby="wallpapers" {
            h2 #wallpapers { (t("Your wallpapers")) }
            @if items.is_empty() { p.text { (t("You have not uploaded any yet.")) } } @else { (rows(items, false)) }
            (upload_form(targets, lit))
        }
    }
}
