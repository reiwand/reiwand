use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use std::path::Path;

impl Daemon {
    pub(crate) fn heard(&self, output: &str, shown: Option<&Path>) -> f64 {
        let uid = shown.and_then(|path| self.state().items.get(path).cloned());
        let item = uid.and_then(|uid| self.store.read(|lib| lib.item(&uid)).ok());
        level(output, item.map_or((None, None), |i| (i.sound_muted, i.sound_volume)))
    }

    pub fn set_sound(&self, uids: &[String], muted: &str, volume: &str) -> Result<()> {
        let (muted, volume) = (word::<bool>(muted)?, word::<u8>(volume)?);
        self.edit_items(uids, |lib, uids| lib.set_sound_many(uids, muted, volume))?;
        self.configure_all();
        Ok(())
    }
}

const CARD: &str = "preview:card-";

fn level(output: &str, (muted, volume): (Option<bool>, Option<u8>)) -> f64 {
    match output.starts_with(CARD) || muted == Some(true) {
        true => 0.0,
        false => volume.map_or(1.0, |v| f64::from(v.min(100)) / 100.0),
    }
}

fn word<T: std::str::FromStr>(text: &str) -> Result<Option<Option<T>>> {
    match text.trim() {
        "" => Ok(None),
        "none" => Ok(Some(None)),
        other => {
            let bad = || invalid(format!("{other:?} is not a valid sound value"));
            other.parse().map(|v| Some(Some(v))).map_err(|_| bad())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compositor::Movie;

    #[test]
    fn a_library_card_plays_without_sound_and_every_other_output_keeps_its_own() {
        for item in [(None, None), (Some(false), Some(80)), (Some(true), None)] {
            let card = Movie::plain("a.webm".into()).heard(level("preview:card-1794", item));
            assert_eq!(card.volume, 0, "a card opens no audio for {item:?}");
        }
        for output in ["DP-1", "preview:editor", "preview:page-lights", "preview:"] {
            assert_eq!(level(output, (None, None)), 1.0, "{output}");
            assert_eq!(level(output, (Some(false), Some(40))), 0.4, "{output}");
            assert_eq!(level(output, (Some(true), Some(40))), 0.0, "{output}");
        }
    }
}
