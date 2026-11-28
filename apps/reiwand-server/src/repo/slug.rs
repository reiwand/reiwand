fn folded(c: char) -> &'static str {
    match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' | 'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => {
            "a"
        }
        'ç' | 'ć' | 'č' | 'Ç' | 'Ć' | 'Č' => "c",
        'ď' | 'đ' | 'Ď' | 'Đ' => "d",
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ę' | 'ě' | 'È' | 'É' | 'Ê' | 'Ë' => "e",
        'ì' | 'í' | 'î' | 'ï' | 'ī' | 'Ì' | 'Í' | 'Î' | 'Ï' => "i",
        'ł' | 'Ł' => "l",
        'ñ' | 'ń' | 'ň' | 'Ñ' | 'Ń' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ő' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => {
            "o"
        }
        'ř' | 'Ř' => "r",
        'ś' | 'š' | 'ş' | 'Ś' | 'Š' | 'Ş' => "s",
        'ť' | 'ţ' | 'Ť' => "t",
        'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ů' | 'ű' | 'Ù' | 'Ú' | 'Û' | 'Ü' => "u",
        'ý' | 'ÿ' | 'Ý' => "y",
        'ź' | 'ž' | 'ż' | 'Ź' | 'Ž' | 'Ż' => "z",
        'ß' => "ss",
        'æ' | 'Æ' => "ae",
        'œ' | 'Œ' => "oe",
        _ => "",
    }
}

pub fn slug(title: &str) -> String {
    let mut out = String::new();
    for c in title.chars() {
        let part =
            if c.is_ascii_alphanumeric() { c.to_ascii_lowercase().to_string() } else { folded(c).into() };
        match part.is_empty() {
            true if !out.ends_with('-') => out.push('-'),
            true => {}
            false => out.push_str(&part),
        }
    }
    let cut: String = out.trim_matches('-').chars().take(32).collect();
    let cut = cut.trim_matches('-');
    if cut.is_empty() { "repo".into() } else { cut.into() }
}

pub fn free_name(title: &str, taken: &[String]) -> String {
    let base = slug(title);
    let mut tried = base.clone();
    let mut n = 1;
    while taken.contains(&tried) {
        n += 1;
        let suffix = format!("-{n}");
        let room = 32 - suffix.len();
        let head: String = base.chars().take(room).collect();
        tried = format!("{}{suffix}", head.trim_end_matches('-'));
    }
    tried
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_become_plain_addresses() {
        assert_eq!(slug("My wallpapers"), "my-wallpapers");
        assert_eq!(slug("  New Year wallpapers!! "), "new-year-wallpapers");
        assert_eq!(slug("Crème — Übung"), "creme-ubung");
        assert_eq!(slug("a___b   c"), "a-b-c");
        assert_eq!(slug("日本語"), "repo");
        assert_eq!(slug("!!!"), "repo");
        assert_eq!(slug(""), "repo");
        assert_eq!(slug(&"x".repeat(60)).len(), 32);
        assert_eq!(slug(&format!("{}-{}", "a".repeat(31), "b")), "a".repeat(31));
        assert!(crate::repo::valid_name(&slug("Ünï cödé 2")));
    }

    #[test]
    fn a_taken_address_gets_a_number() {
        let taken = vec!["my-wallpapers".to_string(), "my-wallpapers-2".to_string()];
        assert_eq!(free_name("My wallpapers", &taken), "my-wallpapers-3");
        assert_eq!(free_name("Other", &taken), "other");
        let long = slug(&"y".repeat(40));
        let name = free_name(&"y".repeat(40), &[long]);
        assert_eq!(name.len(), 32);
        assert!(name.ends_with("-2") && crate::repo::valid_name(&name));
    }
}
