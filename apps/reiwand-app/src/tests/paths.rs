use crate::app;
use crate::bus::url;

#[test]
fn file_uris_become_paths() {
    assert_eq!(app::path("file:///tmp/a%20b.png").as_deref(), Some("/tmp/a b.png"));
    assert_eq!(app::path("/tmp/c.png").as_deref(), Some("/tmp/c.png"));
    assert_eq!([app::path("https://example.org/a.png"), app::path("file://example.org/a.png")], [None, None]);
    assert_eq!(url("/tmp/a b#1.png"), "file:///tmp/a%20b%231.png");
}
