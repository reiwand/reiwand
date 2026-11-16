use std::io::Write;

pub const NOTICES: &str = include_str!("../../../packaging/THIRD-PARTY-NOTICES");

pub fn run() -> anyhow::Result<()> {
    let mut out = std::io::stdout().lock();
    out.write_all(NOTICES.as_bytes())?;
    Ok(())
}
