use std::io::Read;

pub const LICENSES: &str = include_str!("../../../packaging/THIRD-PARTY-NOTICES");

pub fn head(path: String) -> Option<String> {
    let mut text = String::new();
    std::fs::File::open(path).ok()?.take(256 << 10).read_to_string(&mut text).ok().map(|_| text)
}
