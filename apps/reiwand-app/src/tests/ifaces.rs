pub fn iface(line: &str) -> &'static str {
    let ifaces = [
        "Library1",
        "Playlists1",
        "Profiles1",
        "Providers1",
        "Repos1",
        "Inputs1",
        "Sync1",
        "Accounts1",
        "Rgb1",
    ];
    ifaces.into_iter().find(|i| line.contains(&format!("\"{i}\""))).unwrap_or("Daemon1")
}
