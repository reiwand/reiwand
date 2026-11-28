use crate::App;
use crate::band::Shown;
use crate::lang::{current, t, tf};
use maud::{DOCTYPE, Markup, html};

pub const NO_WALLPAPER: &str = "f-none";

pub struct Frame<'a> {
    pub title: &'a str,
    pub line: Option<Markup>,
}

impl<'a> Frame<'a> {
    pub fn new(title: &'a str) -> Self {
        Frame { title, line: None }
    }

    pub fn line(mut self, line: Markup) -> Self {
        self.line = Some(line);
        self
    }
}

fn credit(shown: &Shown) -> String {
    match &shown.by {
        Some(by) => tf("Cover: “{}” by {}, the latest published", &[&shown.title, by]),
        None => tf("Cover: “{}”, the latest published", &[&shown.title]),
    }
}

pub fn render(app: &App, nav: Markup, frame: Frame, body: Markup) -> Markup {
    let shown = app.band();
    let band = if shown.is_some() { "band" } else { "band bare" };
    html! {
        (DOCTYPE)
        html lang=(current().code()) class=[shown.is_none().then_some(NO_WALLPAPER)] {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (frame.title) " · " (app.cfg.name) }
                link rel="icon" type="image/svg+xml" href="/static/icon.svg";
                link rel="stylesheet" href="/static/style.css";
                @if let Some(s) = &shown { link rel="stylesheet" href={ "/band/" (s.sha) ".css" }; }
                script src="/static/copy.js" defer {}
            }
            body {
                a.skip href="#content" { (t("Skip to content")) }
                div.page {
                    header class=(band) {
                        @if let Some(s) = &shown { img.cover-img src={ "/media/" (s.sha) ".jpg" } alt=""; }
                        div.bar {
                            a.brand href="/" { (app.cfg.name) }
                            nav.nav aria-label=(t("Site")) { (nav) }
                        }
                        div.head {
                            h1 { (frame.title) }
                            @if let Some(line) = &frame.line { (line) }
                            @if let Some(s) = &shown { p.cover-by { (credit(s)) } }
                        }
                    }
                    main.column #content { (body) }
                    footer.foot {
                        nav aria-label=(t("Legal")) {
                            a href="/legal" { (t("Legal notice")) }
                            a href="/legal/terms" { (t("Terms")) }
                            a href="/legal/privacy" { (t("Privacy")) }
                            a href="/legal/content" { (t("Content policy")) }
                            a href="/legal/notice" { (t("Report content")) }
                        }
                    }
                }
            }
        }
    }
}
