use zbus::{Result, proxy};

#[proxy(
    interface = "io.github.reiwand.Inputs1",
    default_service = "io.github.reiwand.Daemon",
    default_path = "/io/github/reiwand/Daemon"
)]
pub trait Inputs1 {
    fn grant(&self, scene: &str, input: &str) -> Result<()>;
    fn grant_package(&self, scene: &str, key: &str, input: &str) -> Result<()>;
    fn decide(&self, scene: &str, key: &str, allow: &[&str], deny: &[&str]) -> Result<()>;
    fn revoke(&self, scene: &str, input: &str) -> Result<()>;
    fn get_inputs(&self) -> Result<String>;
    fn set_pointer(&self, x: i32, y: i32) -> Result<()>;
    fn set_clicks(&self, output: &str, on: bool) -> Result<()>;
    fn simulate_media(&self, output: &str, track: &str, cover: &[u8]) -> Result<()>;
    fn simulate_touch(&self, output: &str, x: f64, y: f64, buttons: u32) -> Result<()>;
    fn simulate_trigger(&self, output: &str, trigger: &str) -> Result<()>;
    fn bands(&self, output: &str) -> Result<String>;
}

#[proxy(
    interface = "io.github.reiwand.Publish1",
    default_service = "io.github.reiwand.Daemon",
    default_path = "/io/github/reiwand/Daemon"
)]
pub trait Publish1 {
    fn upload_as(&self, repo: &str, path: &str, visibility: &str) -> Result<u64>;
    fn list_packages(&self) -> Result<String>;
    fn set_visibility(&self, id: &str, version: &str, visibility: &str) -> Result<()>;
    fn list_author_keys(&self) -> Result<String>;
    fn add_author_key(&self, public_key: &str) -> Result<String>;
    fn rotate_author_key(&self, from: &str, public_key: &str, signature: &str) -> Result<String>;
    fn remove_author_key(&self, fingerprint: &str) -> Result<()>;
    fn list_uploads(&self) -> Result<String>;
    fn cancel_upload(&self, id: &str) -> Result<()>;
    fn vote(&self, id: &str, vote: &str) -> Result<String>;
    fn get_votes(&self, ids: &[&str]) -> Result<String>;
}

#[proxy(
    interface = "io.github.reiwand.Rgb1",
    default_service = "io.github.reiwand.Daemon",
    default_path = "/io/github/reiwand/Daemon"
)]
pub trait Rgb1 {
    fn get_status(&self) -> Result<String>;
    fn connect(&self) -> Result<()>;
    fn test(&self, device: i32) -> Result<()>;
    fn get_strip(&self, output: &str) -> Result<(u32, u32, Vec<u8>)>;
}

#[proxy(
    interface = "io.github.reiwand.Playlists1",
    default_service = "io.github.reiwand.Daemon",
    default_path = "/io/github/reiwand/Daemon"
)]
pub trait Playlists1 {
    fn list(&self) -> Result<String>;
    fn get(&self, playlist: &str, offset: u32, limit: u32) -> Result<String>;
    fn create(&self, name: &str, query: &str) -> Result<String>;
    fn rename(&self, playlist: &str, name: &str) -> Result<()>;
    fn set_query(&self, playlist: &str, query: &str) -> Result<()>;
    fn delete(&self, playlist: &str) -> Result<()>;
    fn add_items(&self, playlist: &str, uids: &[&str]) -> Result<()>;
    fn remove_items(&self, playlist: &str, uids: &[&str]) -> Result<()>;
    fn remove_entries(&self, playlist: &str, indexes: &[u32], expected: &[&str]) -> Result<()>;
    fn move_item(&self, playlist: &str, from: u32, to: u32, expected: &str) -> Result<()>;
    fn set_rule(&self, playlist: &str, key: &str, value: &str) -> Result<()>;
    fn add_folder(&self, playlist: &str, path: &str, follow: bool) -> Result<u32>;
    fn rotate(
        &self,
        name: &str,
        uids: &[&str],
        folder: &str,
        follow: bool,
        every: &str,
        output: &str,
    ) -> Result<String>;
    fn start(&self, playlist: &str, output: &str) -> Result<()>;
    fn stop(&self, output: &str) -> Result<()>;
    fn next(&self, output: &str) -> Result<()>;
    fn previous(&self, output: &str) -> Result<()>;
    fn history(&self, output: &str, limit: u32) -> Result<String>;
    fn browse(&self, output: &str, source: &str, steps: i32) -> Result<String>;
    fn browse_apply(&self, output: &str, transition: &str) -> Result<String>;
    fn browse_transition(&self, output: &str, transition: &str) -> Result<String>;
    fn browse_cancel(&self, output: &str) -> Result<String>;
    fn get_browse(&self, output: &str) -> Result<String>;
    #[zbus(signal)]
    fn browse_changed(&self, output: &str, json: &str) -> Result<()>;
}

#[proxy(
    interface = "io.github.reiwand.Accounts1",
    default_service = "io.github.reiwand.Daemon",
    default_path = "/io/github/reiwand/Daemon"
)]
pub trait Accounts1 {
    fn list(&self) -> Result<String>;
    fn sign_in(&self, repo_url: &str) -> Result<(String, String, u64)>;
    #[zbus(name = "Use")]
    fn use_account(&self, repo: &str, account: &str) -> Result<()>;
    fn unlink(&self, repo: &str) -> Result<()>;
    fn sign_out(&self, account: &str) -> Result<()>;
    fn remove(&self, account: &str) -> Result<()>;
}
