use crate::daemon::Daemon;
use crate::error::{Result, failed};
use std::sync::{Arc, Weak};

pub struct Publish(pub Weak<Daemon>);

impl Publish {
    fn d(&self) -> Result<Arc<Daemon>> {
        self.0.upgrade().ok_or_else(|| failed("the daemon is stopping"))
    }
}

#[zbus::interface(name = "io.github.reiwand.Publish1")]
impl Publish {
    #[zbus(out_args("job"))]
    async fn upload(&self, path: String) -> Result<u64> {
        self.d()?.run(move |d| d.upload(&path, "public")).await
    }

    #[zbus(out_args("job"))]
    async fn upload_as(&self, path: String, visibility: String) -> Result<u64> {
        self.d()?.run(move |d| d.upload(&path, &visibility)).await
    }

    #[zbus(out_args("json"))]
    async fn list_packages(&self) -> Result<String> {
        self.d()?.run(|d| d.own_packages()).await
    }

    #[zbus(out_args("json"))]
    async fn list_author_keys(&self) -> Result<String> {
        self.d()?.run(|d| d.list_author_keys()).await
    }

    #[zbus(out_args("fingerprint"))]
    async fn add_author_key(&self, public_key: String) -> Result<String> {
        self.d()?.run(move |d| d.add_author_key(&public_key)).await
    }

    #[zbus(out_args("fingerprint"))]
    async fn rotate_author_key(&self, from: String, public_key: String, signature: String) -> Result<String> {
        self.d()?.run(move |d| d.rotate_author_key(&from, &public_key, &signature)).await
    }

    async fn remove_author_key(&self, fingerprint: String) -> Result<()> {
        self.d()?.run(move |d| d.remove_author_key(&fingerprint)).await
    }

    #[zbus(out_args("json"))]
    async fn list_uploads(&self) -> Result<String> {
        self.d()?.run(|d| d.list_uploads()).await
    }

    async fn cancel_upload(&self, id: String) -> Result<()> {
        self.d()?.run(move |d| d.cancel_upload(&id)).await
    }

    #[zbus(out_args("json"))]
    async fn vote(&self, id: String, vote: String) -> Result<String> {
        self.d()?.run(move |d| d.vote(&id, &vote)).await
    }

    #[zbus(out_args("json"))]
    async fn get_votes(&self, ids: Vec<String>) -> Result<String> {
        self.d()?.run(move |d| d.votes(&ids)).await
    }

    async fn set_visibility(&self, id: String, version: String, visibility: String) -> Result<()> {
        self.d()?.run(move |d| d.set_visibility(&id, &version, &visibility)).await
    }
}
