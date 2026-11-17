use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use crate::jobs::Ctl;
use reiwand_account::{Api, Poll, poll};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

const REFUSED: &str = "the sign-in was refused";
const EXPIRED: &str = "the code expired before it was approved; sign in again";

fn pause(ctl: &Ctl, wait: Duration) -> Result<()> {
    let until = Instant::now() + wait;
    while Instant::now() < until {
        ctl.check()?;
        std::thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

impl Daemon {
    pub(super) fn enroll(
        &self,
        api: &Api,
        code: &str,
        mut interval: u64,
        expires: u64,
        ctl: &Ctl,
    ) -> Result<Value> {
        let deadline = Instant::now() + Duration::from_secs(expires);
        while Instant::now() < deadline {
            pause(ctl, Duration::from_secs(interval))?;
            let answer = block_on(api.post("/api/v1/auth/device/token", json!({"device_code": code})));
            match poll(&mut interval, answer) {
                Poll::Done(token) => return Ok(token),
                Poll::Wait(_) => {}
                Poll::Stop(e) if e.code() == "access_denied" => return Err(failed(REFUSED)),
                Poll::Stop(e) => return Err(sync_error(e)),
            }
        }
        Err(failed(EXPIRED))
    }
}
