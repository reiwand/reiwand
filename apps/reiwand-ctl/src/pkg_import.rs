use crate::cli::Wait;
use crate::daemon::finished;
use crate::input::{path_arg, strs};
use crate::told::{fill, told};
use anyhow::{Result, ensure};
use futures_util::TryFutureExt;
use reiwand_client::Client;
use reiwand_core::model::ImportReport;
use std::future::Future;

pub async fn run(client: &Client, files: &[String], allow_older: bool, wait: &Wait) -> Result<()> {
    let paths = files.iter().map(|file| path_arg(file)).collect::<Result<Vec<_>>>()?;
    waited(client, client.library.import_packages(&strs(&paths), allow_older), wait).await
}

pub async fn waited(
    client: &Client,
    start: impl Future<Output = zbus::Result<u64>>,
    wait: &Wait,
) -> Result<()> {
    let (_, job) = finished(client, start.map_ok(|id| ((), id)), wait).await?;
    let report =
        job.and_then(|job| job.output).and_then(|out| serde_json::from_value::<ImportReport>(out).ok());
    let failed = report.map_or(0, |report| report.failed.len());
    ensure!(failed == 0, "{}", fill(told("some files were not imported (%1)"), &[&failed]));
    Ok(())
}
