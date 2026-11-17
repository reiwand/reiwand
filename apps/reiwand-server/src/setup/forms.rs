use super::view::{stepped, summary};
use crate::App;
use crate::lang::t;
use crate::settings::Access;
use crate::ui::{Choice, Kind, button, checkbox, choices, field, fold, qr};
use maud::{Markup, html};

pub fn account(app: &App, handle: Option<&str>, error: Option<&str>) -> Markup {
    let body = html! {
        (summary(app, 1))
        section {
            h2.titulo-seccion { (t("Administrator account")) }
            p.intro { (t("With this account you get in to manage, review and upload.")) }
            form.formulario.separado method="post" action="/setup/account" {
                (field(t("Handle"), html! {
                    input name="handle" autocomplete="username" required minlength="3" maxlength="32" value=[handle]
                        aria-invalid=[error.map(|_| "true")];
                }, Some(t("3 to 32 characters.")), error))
                (field(t("Password"), html! {
                    input type="password" name="password" autocomplete="new-password" required minlength="10";
                }, Some(t("10 characters or more.")), None))
                div { (button(Kind::Primary, t("Next"))) }
            }
        }
    };
    stepped(app, t("Set up your server"), 1, body)
}

pub fn repository(app: &App, name: Option<&str>) -> Markup {
    let error = name.map(|_| t("Write a name of 1 to 80 characters."));
    let body = html! {
        (summary(app, 2))
        section {
            h2.titulo-seccion { (t("Create the repository")) }
            p.intro { (t("It is the collection the app and the phone see when they connect to this server.")) }
            form.formulario.separado method="post" action="/setup/repository" {
                (field(t("Repository name"), html! {
                    input name="name" required maxlength="80" value=[name] aria-invalid=[error.map(|_| "true")];
                }, None, error))
                (fold(t("Advanced"), html! {
                    (field(t("Address name"), html! { input name="repo" value="main" maxlength="32" pattern="[a-z0-9][a-z0-9\\-]*"; },
                        Some(t("Lowercase letters, digits and hyphens: it forms the address of the repository on this server.")), None))
                    (checkbox(html! { input type="checkbox" name="online_authors" checked; },
                    t("Online authors"),
                    Some(t("Each person adds their author keys from their account, without touching the root key."))))
                }, false))
                div { (button(Kind::Primary, t("Create the repository"))) }
            }
        }
    };
    stepped(app, t("Set up your server"), 2, body)
}

pub fn access(app: &App, code: &str) -> Markup {
    let settings = app.settings();
    let options = [
        Choice {
            value: "closed",
            title: t("Closed"),
            text: t("Nobody signs up on their own. What is uploaded is published at once."),
        },
        Choice {
            value: "invite",
            title: t("By invitation"),
            text: t("Whoever has an invitation from you gets in. What is uploaded is published at once."),
        },
        Choice {
            value: "open",
            title: t("Open"),
            text: t("Anyone can sign up. What new accounts upload waits for your review."),
        },
    ];
    let chosen = match settings.access() {
        Access::Closed => "invite",
        other => other.value(),
    };
    let link = app.cfg.url(&format!("/register?invite={code}"));
    let svg = crate::qr::own_connect_svg(app, Some(code)).unwrap_or_default();
    let name = settings.repo_name.clone().unwrap_or_default();
    let body = html! {
        (summary(app, 3))
        section {
            h2.titulo-seccion { (t("Who gets in")) }
            form.formulario.ancho.separado method="post" action="/setup/access" {
                (choices(t("Who gets in"), "access", &options, chosen))
                div {
                    h3.subtitulo { (t("Your first invitation")) }
                    (qr(&svg, t("QR code with the invitation link"), html! {
                        b { (t("Give it to whoever you want to get in.")) }
                        span.texto { (t("It is shown only this time; you can make more in People.")) }
                        span.dato { (link) }
                    }))
                }
                div { (button(Kind::Primary, t("Finish"))) }
            }
        }
    };
    stepped(app, if name.is_empty() { t("Set up your server") } else { &name }, 3, body)
}
