use crate::auth::{Auth, Role};
use crate::lang::{t, tf};
use crate::ui::Tone;
use crate::ui::{Kind, Row, button, button_named, confirm, field, fold, link_button, rows, status};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

pub fn tools(auth: Option<&Auth>, repo: &str, id: &str, latest: &Value, ask: &str) -> Markup {
    let Some(auth) = auth else { return html! {} };
    let (moderator, admin) = (auth.role >= Role::Moderator, auth.role >= Role::Admin);
    let own = latest["owner"].as_str() == Some(auth.handle.as_str());
    if !moderator && !own {
        return html! {};
    }
    let here = format!("/r/{repo}/packages/{id}");
    let api = format!("/api/v1/r/{repo}");
    let mut items = Vec::new();
    if moderator {
        items.push(shown(&api, &here, id, latest["hidden"].as_str()));
        items.push(removal(&api, repo, id));
    }
    if own {
        let ask_it = tf(
            "Withdraw \"{}\" for good? Every version leaves this repository, its files are deleted and the devices that installed it stop showing it at their next refresh. This cannot be undone.",
            &[&text(latest, "title")],
        );
        let action = format!("{api}/me/packages/{id}?_method=DELETE&back=/account");
        items.push(ending(
            t("Withdraw this wallpaper for good"),
            t("Withdraw"),
            ask == "withdraw",
            &here,
            "withdraw",
            &ask_it,
            &action,
        ));
    }
    if admin {
        let ask_it = tf(
            "Delete \"{}\" for good? Every version is revoked in the signed index, its files and votes are deleted and the devices that installed it stop showing it at their next refresh. The package id stays with its owner. This cannot be undone.",
            &[&text(latest, "title")],
        );
        let action = format!("{api}/admin/packages/{id}?_method=DELETE&back=/r/{repo}/");
        items.push(ending(
            t("Delete this wallpaper"),
            t("Delete"),
            ask == "delete",
            &here,
            "delete",
            &ask_it,
            &action,
        ));
    }
    html! {
        section.section aria-labelledby="manage" {
            h2 #manage { (t("Manage")) }
            (rows(items, false))
        }
    }
}

fn shown(api: &str, here: &str, id: &str, hidden: Option<&str>) -> Row {
    let row = Row::new(html! { (t("Catalog")) });
    match hidden {
        Some(reason) => row.value(html! { (status(Tone::Warn, t("Out of the catalog"))) " · " (reason) }).action(html! {
            form method="post" action={ (api) "/mod/hidden/" (id) "?_method=DELETE&back=" (here) } {
                (button(Kind::Plain, t("Return to the catalog")))
            }
        }),
        None => row.value(html! { (status(Tone::Good, t("Listed"))) }).more(fold(
            t("Withdraw from the catalog"),
            html! {
                form.form method="post" action={ (api) "/mod/hidden?back=" (here) } {
                    input type="hidden" name="id" value=(id);
                    (field(t("Reason sent to the author"), html! { textarea name="reason" rows="2" minlength="10" required {} }, None, None))
                    (button(Kind::Plain, t("Withdraw")))
                }
            },
            false,
        )),
    }
}

fn removal(api: &str, repo: &str, id: &str) -> Row {
    let help = t(
        "Every published version is revoked, so devices stop showing it; the files are kept for a counter-notice, and a moderator can restore them from its case.",
    );
    Row::new(html! { (t("Withdraw from the repository")) }).more(fold(
        t("Withdraw from the repository and from devices"),
        html! {
            form.form method="post" action={ (api) "/mod/removed?back=/r/" (repo) "/" } {
                input type="hidden" name="id" value=(id);
                (field(t("Statement sent to the author"), html! { textarea name="statement" rows="3" minlength="10" required {} }, Some(help), None))
                (button(Kind::Plain, t("Withdraw")))
            }
        },
        false,
    ))
}

fn ending(name: &str, verb: &str, asking: bool, here: &str, key: &str, ask: &str, action: &str) -> Row {
    let row = Row::new(html! { (name) });
    if asking {
        let buttons = html! {
            (button_named(Kind::Danger, "confirm", "true", verb))
            (link_button(Kind::Plain, &format!("{here}#manage"), t("Cancel")))
        };
        row.confirm(confirm(ask, t("Confirm"), action, buttons)).lit()
    } else {
        row.action(link_button(Kind::Danger, &format!("{here}?ask={key}#manage"), &format!("{verb}…")))
    }
}
