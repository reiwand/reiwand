use super::{Kind, Tone};
use crate::icons::icon;
use maud::{Markup, html};

pub fn notice(tone: Tone, title: &str, body: Markup, actions: Option<Markup>) -> Markup {
    html! {
        div class={ "note " (tone.class()) } role=(if tone == Tone::Bad { "alert" } else { "status" }) {
            (icon(tone.icon()))
            div.note-body {
                b { (title) }
                (body)
                @if let Some(a) = actions { div.actions { (a) } }
            }
        }
    }
}

pub fn empty(title: &str, first_step: Markup) -> Markup {
    html! { div.empty { b { (title) } p { (first_step) } } }
}

pub fn fold(summary: &str, body: Markup, big: bool) -> Markup {
    html! { details.more.large[big] { summary { (summary) } div.more-body { (body) } } }
}

pub fn fix_fold(summary: &str, body: Markup, open: bool) -> Markup {
    html! { details.more.fix open[open] { summary { (summary) } div.more-body { (body) } } }
}

pub fn copyable(text: &str, label: &str, done: &str) -> Markup {
    copyable_with(text, label, done, Kind::Plain)
}

pub fn copyable_with(text: &str, label: &str, done: &str, kind: Kind) -> Markup {
    html! {
        div.copy {
            code.fact { (text) }
            button class=(kind.class()) type="button" hidden data-copied=(done) { (icon("copy")) span data-label { (label) } }
        }
    }
}
