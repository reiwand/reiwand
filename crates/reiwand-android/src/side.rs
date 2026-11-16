use crate::state::{Saved, held, update};
use crate::{Error, said, vault};
use reiwand_sync::client::{Fault, Fields, Side};
use reiwand_sync::{MIN_PASSPHRASE, Secrets};

pub(crate) struct Phone;

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
        last_reply: Some(&mut s.last_reply),
        warning: &mut s.warning,
        peers: &mut s.peers,
        pinned: &mut s.pinned,
        members: &mut s.members,
        replica: &mut s.replica,
        quarantine: &mut s.quarantine,
        unapplied: &mut s.unapplied,
        recovery: &mut s.recovery,
        recovery_stale: Some(&mut s.recovery_stale),
    }
}

impl Side for Phone {
    type Error = Error;
    const KEEPS_GOING_WITHOUT_ESCROW: bool = true;

    fn read<T>(&self, look: impl FnOnce(&mut Fields<'_>) -> T) -> T {
        held(|s| look(&mut fields(s)))
    }

    fn update(&self, change: impl FnOnce(&mut Fields<'_>)) -> Result<(), Error> {
        update(|s| change(&mut fields(s)))
    }

    async fn keep(&self, secrets: &Secrets) -> Result<(), Error> {
        vault::store(secrets)
    }
}

impl From<Fault> for Error {
    fn from(fault: Fault) -> Self {
        match fault {
            Fault::Sync(e) => e.into(),
            Fault::NoKey => said(
                "android.phone_sync_key",
                "this phone has no sync key yet; join from a linked device",
                &[],
            ),
            Fault::HeldKey => said("android.phone_holds_key", "this phone already holds the sync key", &[]),
            Fault::NeedsEncryption => {
                said("android.phrase_encryption", "a recovery passphrase needs end-to-end encryption", &[])
            }
            Fault::ShortPhrase => said(
                "android.phrase_short",
                "a recovery passphrase has at least %1 characters",
                &[&MIN_PASSPHRASE],
            ),
            Fault::SamePhrase => said(
                "android.phrase_same",
                "use another passphrase: the one before may be known to a revoked device",
                &[],
            ),
            Fault::NoEscrow => said("android.no_escrow", "the account has no recovery escrow", &[]),
            Fault::EscrowReplayed { epoch, seen } => said(
                "android.escrow_replayed",
                "the recovery escrow holds key epoch %1, older than epoch %2 that this phone already saw: the server may replay an old escrow; join from a device",
                &[&epoch, &seen],
            ),
            Fault::EscrowOldKey => said(
                "android.escrow_old_key",
                "the recovery escrow holds an older key of the account; join from a device",
                &[],
            ),
            Fault::EncryptionChanged => said(
                "android.server_changed_encryption",
                "the server changed the encryption of this account; nothing was synced",
                &[],
            ),
            Fault::KeyChanged => said(
                "android.sync_key_account",
                "the sync key of the account changed: waiting for the new key, or join again",
                &[],
            ),
            Fault::KeyNotStored => said("android.new_key_could", "the new key could not be stored", &[]),
            Fault::PinBroken => said("android.pinned_device_key", "a pinned device key is broken", &[]),
            Fault::PlainRotation => {
                said("android.rotate_plain", "sync is not encrypted: a new key would protect nothing", &[])
            }
            Fault::RotatedElsewhere => {
                said("android.rotated_elsewhere", "another device replaced the key first", &[])
            }
            Fault::NeedsPlain => said(
                "android.account_syncs_without",
                "the account syncs without end-to-end encryption; link with plain text only if you accept that",
                &[],
            ),
            Fault::OtherEncryption => said(
                "android.server_set_up",
                "the server set up sync with another encryption than asked",
                &[],
            ),
            Fault::NoDevice(wanted) => said("android.no_device", "no device %1 on the server", &[&wanted]),
            Fault::OwnDevice => {
                said("android.choose_another_device", "choose another device than this phone", &[])
            }
        }
    }
}
