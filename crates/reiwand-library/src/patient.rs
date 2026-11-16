use crate::{Error, Result};

pub(crate) fn patient(mut step: impl FnMut() -> Result<()>) -> Result<()> {
    let mut waited = 0;
    loop {
        match step() {
            Err(Error::Sqlite(rusqlite::Error::SqliteFailure(failure, _)))
                if waited < 100
                    && matches!(
                        failure.code,
                        rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
                    ) =>
            {
                waited += 1;
                std::thread::sleep(std::time::Duration::from_millis(20 + waited));
            }
            other => return other,
        }
    }
}
