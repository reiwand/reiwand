const OWN: &str = "wpak: wpak-user: data: blob:";

pub fn csp(network: &[String], mask_gpu: bool) -> String {
    let base = reiwand_format::page::csp(OWN, "wpak:", network);
    let base = match mask_gpu {
        true => base.replace("worker-src wpak: wpak-user: data: blob:", "worker-src 'none'"),
        false => base,
    };
    format!("{base}; webrtc 'block'")
}

pub fn languages(real: &[String], hidden: bool) -> Vec<String> {
    let tag = |l: &String| l.split(['.', '@']).next().unwrap_or_default().replace('_', "-");
    let mut tags: Vec<String> =
        real.iter().map(tag).filter(|l| !l.is_empty() && l != "C" && l != "POSIX").collect();
    tags.dedup();
    match tags.is_empty() || hidden {
        true => vec!["en-US".into()],
        false => tags,
    }
}

pub use reiwand_format::page::mime;
