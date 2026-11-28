use crate::icons::icon;
use maud::{Markup, html};

pub fn steps(label: &str, names: &[&str], current: usize) -> Markup {
    html! {
        ol.steps aria-label=(label) {
            @for (i, name) in names.iter().enumerate() {
                @if i < current {
                    li { (icon("check-circle-fill")) (name) }
                } @else if i == current {
                    li.current aria-current="step" { (name) }
                } @else {
                    li { (name) }
                }
            }
        }
    }
}

pub struct Choice<'a> {
    pub value: &'a str,
    pub title: &'a str,
    pub text: &'a str,
}

pub fn choices(legend: &str, name: &str, options: &[Choice], chosen: &str) -> Markup {
    html! {
        fieldset.choices {
            legend.hidden { (legend) }
            @for c in options {
                label.choice {
                    input type="radio" name=(name) value=(c.value) checked[c.value == chosen];
                    span { b { (c.title) } small { (c.text) } }
                }
            }
        }
    }
}
