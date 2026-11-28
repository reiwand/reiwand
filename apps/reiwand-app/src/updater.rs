use crate::backend::qobject::Backend;

impl Backend {
    /// Whether this copy is the AppImage's, which installs new versions itself (`update.sh` sits
    /// next to the `bin/` folder of its programs); packages update through their own manager.
    pub fn self_updating(&self) -> bool {
        let exe = std::env::current_exe().ok();
        exe.as_deref().and_then(|e| e.parent()?.parent()).is_some_and(|root| root.join("update.sh").is_file())
    }
}
