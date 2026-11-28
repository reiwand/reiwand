use crate::icons::icon;
use maud::{Markup, html};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Plain,
    Primary,
    Danger,
    Link,
}

impl Kind {
    pub fn class(self) -> &'static str {
        match self {
            Kind::Plain => "button",
            Kind::Primary => "button primary",
            Kind::Danger => "button danger",
            Kind::Link => "button link",
        }
    }
}

pub fn button(kind: Kind, label: &str) -> Markup {
    html! { button class=(kind.class()) { (label) } }
}

pub fn button_named(kind: Kind, name: &str, value: &str, label: &str) -> Markup {
    html! { button class=(kind.class()) name=(name) value=(value) { (label) } }
}

pub fn link_button(kind: Kind, href: &str, label: &str) -> Markup {
    html! { a class=(kind.class()) href=(href) { (label) } }
}

pub fn disabled_button(label: &str, reason: &str) -> Markup {
    html! {
        button.button disabled { (label) }
        span.why { (icon("info")) (reason) }
    }
}

pub fn form_actions(buttons: Markup) -> Markup {
    html! { div.actions { (buttons) } }
}

pub fn field(label: &str, input: Markup, help: Option<&str>, error: Option<&str>) -> Markup {
    html! {
        label.field.bad[error.is_some()] {
            span.label { (label) }
            (input)
            @if let Some(help) = help { span.help { (help) } }
            @if let Some(error) = error { span.field-error { (icon("warning-octagon-fill")) (error) } }
        }
    }
}

pub fn checkbox(input: Markup, label: &str, small: Option<&str>) -> Markup {
    html! {
        label.checkbox {
            (input)
            span { (label) @if let Some(small) = small { small { (small) } } }
        }
    }
}

pub fn file_picker(input: Markup, button_label: &str, empty: &str) -> Markup {
    html! {
        label.file {
            (input)
            span.button { (icon("upload-simple")) (button_label) }
            span.file-name { (empty) }
        }
    }
}
