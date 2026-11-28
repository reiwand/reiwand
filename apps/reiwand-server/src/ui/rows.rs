use super::Tone;
use crate::icons::icon;
use maud::{Markup, html};

pub struct Row {
    pub name: Markup,
    pub value: Option<Markup>,
    pub action: Option<Markup>,
    pub lit: bool,
    pub thumb: Option<String>,
    pub confirm: Option<Markup>,
    pub more: Option<Markup>,
}

impl Row {
    pub fn new(name: Markup) -> Self {
        Row { name, value: None, action: None, lit: false, thumb: None, confirm: None, more: None }
    }

    pub fn value(mut self, value: Markup) -> Self {
        self.value = Some(value);
        self
    }

    pub fn action(mut self, action: Markup) -> Self {
        self.action = Some(action);
        self
    }

    pub fn thumb(mut self, src: String) -> Self {
        self.thumb = Some(src);
        self
    }

    pub fn confirm(mut self, ask: Markup) -> Self {
        self.confirm = Some(ask);
        self
    }

    pub fn more(mut self, more: Markup) -> Self {
        self.more = Some(more);
        self
    }

    pub fn lit(mut self) -> Self {
        self.lit = true;
        self
    }
}

pub fn rows(items: Vec<Row>, compact: bool) -> Markup {
    html! {
        ul.rows.compact[compact] {
            @for r in items {
                li.row.lit[r.lit] {
                    @if let Some(src) = &r.thumb { img.thumbnail src=(src) alt="" loading="lazy"; }
                    div.row-text {
                        div.row-name { (r.name) }
                        @if let Some(v) = r.value { div.row-value { (v) } }
                    }
                    @if let Some(a) = r.action { div.row-action { (a) } }
                    @if let Some(c) = r.confirm { (c) }
                    @if let Some(m) = r.more { (m) }
                }
            }
        }
    }
}

pub fn status(tone: Tone, text: &str) -> Markup {
    html! { span class={ "status " (tone.class()) } { (icon(tone.icon())) (text) } }
}

pub fn status_line(tone: Tone, text: &str, detail: Option<&str>) -> Markup {
    html! {
        p class={ "status-line " (tone.class()) } {
            (icon(tone.icon())) span.text-line { (text) }
            @if let Some(d) = detail { span.faint { (d) } }
        }
    }
}

pub fn pairs(items: &[(&str, Markup)]) -> Markup {
    html! { dl.pairs { @for (k, v) in items { dt { (k) } dd { (v) } } } }
}

pub fn quiet_line(text: &str) -> Markup {
    html! { p.status-line { span.faint { (text) } } }
}
