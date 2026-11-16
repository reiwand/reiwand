use super::link::NEEDS_PLAIN;
use super::saved::Saved;
use super::sync_error;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, invalid, not_found};
use reiwand_sync::client::{Fault, Fields, Side};
use reiwand_sync::{MIN_PASSPHRASE, Secrets};
use zeroize::Zeroizing;

fn fields(s: &mut Saved) -> Fields<'_> {
    Fields {
        owner: &s.owner,
        device: &s.device,
        label: &s.label,
        encrypted: &mut s.encrypted,
        plaintext: &mut s.plaintext,
        epoch: &mut s.epoch,
        ready: &mut s.ready,
        stale: &mut s.stale,
        sync_off: &mut s.sync_off,
        last_error: &mut s.last_error,
        last_reply: None,
        warning: &mut s.warning,
        peers: &mut s.peers,
        pinned: &mut s.pinned,
        members: &mut s.members,
        replica: &mut s.replica,
        quarantine: &mut s.quarantine,
        unapplied: &mut s.unapplied,
        recovery: &mut s.recovery,
        recovery_stale: None,
    }
}

impl Side for Daemon {
    type Error = Error;
    const KEEPS_GOING_WITHOUT_ESCROW: bool = false;

    fn read<T>(&self, look: impl FnOnce(&mut Fields<'_>) -> T) -> T {
        look(&mut fields(&mut self.saved()))
    }

    fn update(&self, change: impl FnOnce(&mut Fields<'_>)) -> Result<()> {
        Daemon::update(self, |s| change(&mut fields(s)))
    }

    async fn keep(&self, secrets: &Secrets) -> Result<()> {
        let text = Zeroizing::new(serde_json::to_string(secrets).map_err(|e| failed(e.to_string()))?);
        self.server_keyring()?.store(super::secrets::ID, &text).await
    }
}

impl From<Fault> for Error {
    fn from(fault: Fault) -> Self {
        match fault {
            Fault::Sync(e) => sync_error(e),
            Fault::NoKey => failed("this computer has no sync key yet; join from a linked device"),
            Fault::HeldKey => invalid("this computer already holds the sync key"),
            Fault::NeedsEncryption => failed("a recovery passphrase needs end-to-end encryption"),
            Fault::ShortPhrase => {
                invalid(format!("a recovery passphrase has at least {MIN_PASSPHRASE} characters"))
            }
            Fault::SamePhrase => invalid(crate::words::same_passphrase()),
            Fault::NoEscrow => failed("the account has no recovery escrow"),
            Fault::EscrowReplayed { epoch, seen } => failed(format!(
                "the recovery escrow holds key epoch {epoch}, older than epoch {seen} that this computer already \
                 saw: the server may replay an old escrow; join from a device"
            )),
            Fault::EscrowOldKey => {
                failed("the recovery escrow holds an older key of the account; join from a device")
            }
            Fault::EncryptionChanged => {
                failed("the server changed the encryption of this account; nothing was synced")
            }
            Fault::KeyChanged => {
                failed("the sync key of the account changed: waiting for the new key, or join again")
            }
            Fault::KeyNotStored => failed("the new key could not be stored"),
            Fault::PinBroken => failed("a pinned device key is broken"),
            Fault::PlainRotation => failed("sync is not encrypted: a new key would protect nothing"),
            Fault::RotatedElsewhere => failed("another device replaced the key first"),
            Fault::NeedsPlain => failed(NEEDS_PLAIN),
            Fault::OtherEncryption => failed("the server set up sync with another encryption than asked"),
            Fault::NoDevice(wanted) => not_found(format!("no device {wanted} on the server")),
            Fault::OwnDevice => invalid("choose another device than this one"),
        }
    }
}
