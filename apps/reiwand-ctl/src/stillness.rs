use crate::told::{told, tr};
use reiwand_core::model::{OutputState, StillReason};

pub fn note(o: &OutputState) -> Option<String> {
    let text = match o.still_reason? {
        StillReason::Covered => told("it is still because a window covers the screen"),
        StillReason::Battery => told("it is still because the computer is on battery"),
        StillReason::Locked => told("it is still because the screen is locked"),
        StillReason::Paused => told("it is still because it was paused (reiwand-ctl resume)"),
    };
    Some(tr(text))
}

#[cfg(test)]
mod tests {
    use super::note;
    use reiwand_core::model::StillReason;

    #[test]
    fn each_reason_has_its_words_and_a_playing_screen_has_none() {
        let mut o = crate::fixture::output(serde_json::json!({}));
        assert_eq!(note(&o), None);
        let words: Vec<_> =
            [StillReason::Covered, StillReason::Battery, StillReason::Locked, StillReason::Paused]
                .into_iter()
                .map(|r| {
                    o.still_reason = Some(r);
                    note(&o).unwrap()
                })
                .collect();
        assert!(words[0].contains("window") && words[1].contains("battery"));
        assert!(words[2].contains("locked") && words[3].contains("paused"));
    }
}
