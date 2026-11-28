use crate::carry::Options;
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use std::sync::{Arc, Weak};

pub struct Carry(pub Weak<Daemon>);

impl Carry {
    fn d(&self) -> Result<Arc<Daemon>> {
        self.0.upgrade().ok_or_else(|| failed("the daemon is stopping"))
    }
}

#[zbus::interface(name = "io.github.reiwand.Carry1")]
impl Carry {
    #[zbus(out_args("job"))]
    async fn export(&self, path: String, options_json: String) -> Result<u64> {
        self.d()?.run(move |d| d.export_carry(&path, Options::parse(&options_json)?)).await
    }

    #[zbus(out_args("json"))]
    async fn preview_import(&self, path: String, password: String) -> Result<String> {
        self.d()?.run(move |d| d.carry_preview(&path, &password)).await
    }

    #[zbus(out_args("job"))]
    async fn import_chosen(&self, path: String, password: String, choices_json: String) -> Result<u64> {
        self.d()?.run(move |d| d.import_carry(&path, &password, &choices_json)).await
    }
}
