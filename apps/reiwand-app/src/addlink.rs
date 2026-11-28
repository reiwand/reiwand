use percent_encoding::percent_decode_str;

const LONGEST: usize = 2048;

pub fn address(uri: &str) -> Option<String> {
    let rest = uri.strip_prefix("reiwand://add")?;
    let query = rest.strip_prefix('/').unwrap_or(rest).strip_prefix('?')?;
    let value = query.split('&').find_map(|pair| pair.strip_prefix("url="))?;
    let text = percent_decode_str(value).decode_utf8().ok()?.trim().to_owned();
    (!text.is_empty() && text.len() <= LONGEST && !text.chars().any(char::is_control)).then_some(text)
}

pub fn page(uris: &[String]) -> Option<String> {
    let link = uris.iter().find_map(|uri| address(uri)).map(|url| format!("add:{url}"));
    let files: Vec<String> = uris.iter().filter_map(|uri| crate::app::path(uri)).collect();
    link.or_else(|| (!files.is_empty()).then(|| format!("open:{}", serde_json::json!(files))))
}

#[cfg(test)]
mod tests {
    use super::{address, page};

    #[test]
    fn the_address_is_the_decoded_url_parameter() {
        let link = "reiwand://add?url=https%3A%2F%2Fhome.example%2Fwallpapers%2F";
        assert_eq!(address(link).as_deref(), Some("https://home.example/wallpapers/"));
        assert_eq!(address("reiwand://add/?x=1&url=file%3A%2F%2F%2Frepo").as_deref(), Some("file:///repo"));
    }

    #[test]
    fn anything_else_is_not_an_add_link() {
        for uri in [
            "reiwand://add",
            "reiwand://add?url=",
            "reiwand://add?url=%00",
            "reiwand://other?url=x",
            "file:///a.png",
            "/a.png",
        ] {
            assert_eq!(address(uri), None, "{uri}");
        }
        assert_eq!(address(&format!("reiwand://add?url={}", "a".repeat(3000))), None);
    }

    #[test]
    fn opened_files_are_sent_to_the_importer_as_paths() {
        let uris = ["file:///srv/a/my%20pond.wpak".to_owned(), "/srv/a/b.png".to_owned(), "x".to_owned()];
        assert_eq!(page(&uris).as_deref(), Some(r#"open:["/srv/a/my pond.wpak","/srv/a/b.png"]"#));
    }

    #[test]
    fn the_page_request_carries_the_address() {
        let uris = ["/a.png".to_owned(), "reiwand://add?url=https%3A%2F%2Fa.example%2Fr".to_owned()];
        assert_eq!(page(&uris).as_deref(), Some("add:https://a.example/r"));
        assert_eq!(page(&uris[..1]).as_deref(), Some(r#"open:["/a.png"]"#));
        assert_eq!(page(&[]), None);
    }
}
