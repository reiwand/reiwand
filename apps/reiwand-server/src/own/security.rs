use crate::icons::icon;
use crate::lang::t;
use crate::ui::{Kind, button, checkbox, field, fold, form_actions, subheading};
use maud::{Markup, html};

fn password() -> Markup {
    html! {
        form.form method="post" action="/api/v1/me/password?back=/account" {
            (subheading(t("Change the password")))
            (field(t("Current password"), html! { input type="password" name="current" autocomplete="current-password"; }, None, None))
            (field(t("New password"), html! { input type="password" name="new" autocomplete="new-password" minlength="10" required; },
                Some(t("10 characters or more.")), None))
            (checkbox(html! { input type="checkbox" name="devices" value="true"; }, t("Also sign out every device"), None))
            p.text { (t("A new password signs out every other session and deletes your access tokens. Your \
                devices stay linked unless you also sign them out.")) }
            p.text { (t("Signed-out devices have to sign in again.")) }
            (form_actions(button(Kind::Plain, t("Change password"))))
        }
    }
}

pub fn security() -> Markup {
    html! {
        section.section aria-labelledby="password" {
            h2 #password { (t("Password and data")) }
            (password())
            div.actions {
                a.button href="/api/v1/me/export" { (icon("download-simple")) (t("Download all my data")) }
                span.why { (t("A file with your account, your wallpapers and your devices.")) }
            }
            (fold(t("Delete my account"), html! {
                form.form method="post" action="/api/v1/me?_method=DELETE&back=/" {
                    p.text { (t("Deletes the account and its data.")) }
                    (field(t("Password"), html! { input type="password" name="password" autocomplete="current-password"; }, None, None))
                    (form_actions(button(Kind::Danger, t("Delete my account"))))
                }
            }, false))
        }
    }
}
