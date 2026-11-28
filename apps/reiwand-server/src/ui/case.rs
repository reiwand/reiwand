use crate::icons::icon;
use maud::{Markup, html};

pub fn case_head(src: &str, alt: &str, title: &str, foot: &str) -> Markup {
    html! {
        img.case-cover src=(src) alt=(alt);
        h2.case-title { (title) }
        p.case-foot { (foot) }
    }
}

pub fn back_link(href: &str, label: &str) -> Markup {
    html! { a.back href=(href) { (icon("arrow-left")) (label) } }
}

pub fn subheading(text: &str) -> Markup {
    html! { h3.subtitle { (text) } }
}

pub fn confirm(question: &str, aria: &str, action: &str, actions: Markup) -> Markup {
    html! {
        form.confirm method="post" action=(action) role="group" aria-label=(aria) {
            p { (question) }
            div.actions { (actions) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_case_pieces_carry_their_kit_classes() {
        let head = case_head("/media/a.jpg", "Preview", "Dawn", "Scene").into_string();
        for part in ["class=\"case-cover\"", "class=\"case-title\"", "class=\"case-foot\"", "alt=\"Preview\""]
        {
            assert!(head.contains(part), "{part}");
        }
        assert!(back_link("/mod", "Review").into_string().contains("class=\"back\""));
        let ask = confirm("Sure?", "Confirm", "/x", html! { button { "Yes" } }).into_string();
        assert!(
            ask.contains("class=\"confirm\"")
                && ask.contains("role=\"group\"")
                && ask.contains("action=\"/x\"")
        );
    }
}
