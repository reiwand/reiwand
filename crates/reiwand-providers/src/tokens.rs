use crate::fetch::origin;
use reqwest::Url;

#[derive(Default)]
pub(crate) struct Tokens(Vec<(String, String, String)>);

impl Tokens {
    pub(crate) fn add(&mut self, base: &Url, token: &str) {
        let mut path = base.path().to_owned();
        if !path.ends_with('/') {
            path.push('/');
        }
        let home = origin(base);
        self.0.retain(|(o, p, _)| !(*o == home && *p == path));
        self.0.push((home, path, token.to_owned()));
    }

    pub(crate) fn of(&self, url: &Url) -> Option<&str> {
        let home = origin(url);
        let found = self.0.iter().filter(|(o, p, _)| *o == home && url.path().starts_with(p.as_str()));
        found.max_by_key(|(_, p, _)| p.len()).map(|(_, _, token)| token.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_goes_only_below_its_repository_on_its_origin() {
        let mut tokens = Tokens::default();
        tokens.add(&Url::parse("https://one.example/r/shelf").unwrap(), "t");
        let of = |u: &str| tokens.of(&Url::parse(u).unwrap()).map(String::from);
        assert_eq!(of("https://one.example/r/shelf/entry.json").as_deref(), Some("t"));
        assert_eq!(of("https://one.example/r/shelfy/entry.json"), None);
        assert_eq!(of("https://two.example/r/shelf/entry.json"), None);
        assert_eq!(of("http://one.example/r/shelf/entry.json"), None);
    }
}
