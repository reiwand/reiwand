use crate::cli::Wait;
use crate::daemon::finished;
use crate::input::{path_arg, strs};
use crate::told::{fill, told};
use anyhow::{Result, ensure};
use futures_util::TryFutureExt;
use reiwand_client::Client;

pub async fn run(client: &Client, files: &[String], allow_older: bool, wait: &Wait) -> Result<()> {
    let paths = files.iter().map(|file| path_arg(file)).collect::<Result<Vec<_>>>()?;
    let paths = strs(&paths);
    let start = client.library.import_packages(&paths, allow_older).map_ok(|id| ((), id));
    let (_, job) = finished(client, start, wait).await?;
    let failed = job.and_then(|job| job.output).map_or(0, |out| out["failed"].as_array().map_or(0, Vec::len));
    ensure!(failed == 0, "{}", fill(told("%1 packages were not imported"), &[&failed]));
    Ok(())
}
