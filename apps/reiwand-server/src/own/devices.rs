use crate::db::now;
use crate::icons::icon;
use crate::lang::{since, t, tf};
use crate::ui::{Kind, Row, button, confirm, link_button, rows};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

fn name(d: &Value) -> String {
    match text(d, "name").strip_prefix("Device ") {
        Some(number) => tf("Device {}", &[&number]),
        None => text(d, "name").to_string(),
    }
}

fn platform(d: &Value) -> String {
    let raw = text(d, "platform");
    let mut chars = raw.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

fn row(d: &Value, asking: bool) -> Row {
    let id = text(d, "id");
    let age = (now() - d["created_at"].as_i64().unwrap_or_default()) / 1000;
    let value = html! { (tf("{} · linked {}", &[&platform(d), &since(age)])) };
    let row = Row::new(html! { (name(d)) }).value(value);
    if asking {
        let ask = tf(
            "Remove \"{}\"? It loses its access to your account and, to use it again, you will have to sign in again.",
            &[&name(d)],
        );
        let action = format!("/api/v1/me/devices/{id}?_method=DELETE&back=/account");
        let buttons = html! {
            (button(Kind::Danger, t("Remove")))
            (link_button(Kind::Plain, "/account#dispositivos", t("Cancel")))
        };
        row.confirm(confirm(&ask, t("Confirm"), &action, buttons)).lit()
    } else {
        row.action(
            html! { (link_button(Kind::Danger, &format!("/account?remove={id}#dispositivos"), t("Remove"))) },
        )
    }
}

pub fn devices(list: &[Value], asking: &str) -> Markup {
    let items: Vec<Row> = list.iter().map(|d| row(d, text(d, "id") == asking)).collect();
    html! {
        section.seccion aria-labelledby="dispositivos" {
            h2 #dispositivos { (t("Devices")) }
            @if items.is_empty() {
                p.texto { (t("None yet. A device that signs in with its code shows up here.")) }
            } @else { (rows(items, false)) }
            div.acciones { a.boton href="/link" { (icon("link")) (t("Sign in a device with its code")) } }
        }
    }
}
