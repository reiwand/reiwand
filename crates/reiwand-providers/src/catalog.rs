pub struct Kind {
    pub name: &'static str,
    pub hosts: &'static [&'static str],
    pub secret: Option<(&'static str, bool)>,
}

const fn of(
    name: &'static str,
    hosts: &'static [&'static str],
    secret: Option<(&'static str, bool)>,
) -> Kind {
    Kind { name, hosts, secret }
}

const CATALOG: [Kind; 7] = [
    of("bing", &["www.bing.com"], None),
    of("apod", &["api.nasa.gov", "apod.nasa.gov"], Some(("api_key", false))),
    of("wikimedia", &["commons.wikimedia.org", "upload.wikimedia.org"], None),
    of("wallhaven", &["wallhaven.cc", "w.wallhaven.cc"], Some(("api_key", false))),
    of("webdav", &[], Some(("password", true))),
    of("nextcloud", &[], Some(("password", true))),
    of("immich", &[], Some(("api_key", true))),
];

pub fn catalog() -> &'static [Kind] {
    &CATALOG
}

pub fn find(name: &str) -> Option<&'static Kind> {
    CATALOG.iter().find(|k| k.name == name)
}
