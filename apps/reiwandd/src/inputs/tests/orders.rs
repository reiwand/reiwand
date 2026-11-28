use super::*;
use std::collections::{BTreeMap, BTreeSet};

const CARD: &str = "preview:card-1";

#[derive(Clone, Copy, Debug)]
struct Moment {
    covered: bool,
    streaming: bool,
    preview: bool,
}

const fn moment(bits: u8) -> Moment {
    Moment { covered: bits & 1 != 0, streaming: bits & 2 != 0, preview: bits & 4 != 0 }
}

fn tune() -> SceneInputs {
    SceneInputs { media: true, ..scene(SEA, "sha256:aa", false, true) }
}

fn show(hub: &Hub, now: Moment) {
    let screen = View {
        active: !now.covered,
        covered: now.covered,
        unseen: !now.streaming,
        rect: Some([0.0, 0.0, 100.0, 50.0]),
        clicks: false,
    };
    let mut views = BTreeMap::from([("DP-1".to_owned(), screen)]);
    {
        let mut control = lock(&hub.control);
        match now.preview {
            true => control.shown.insert(CARD.into(), tune()),
            false => control.shown.remove(CARD),
        };
    }
    if now.preview {
        views.insert(CARD.into(), View { covered: false, active: true, unseen: false, ..screen });
    }
    hub.update(Some((views, false)));
}

fn real(hub: &Hub, input: Input) -> BTreeSet<String> {
    lock(&hub.control).wants(input).into_iter().filter(|o| o == "DP-1").collect()
}

#[test]
fn the_screen_gets_its_inputs_back_in_every_order_of_cover_stream_and_preview() {
    for from in 0..8 {
        for to in 0..8 {
            let hub = hub(Arc::default());
            {
                let mut control = lock(&hub.control);
                control.grants = bound("sha256:aa", &[Input::Media, Input::Pointer]);
                control.shown.insert("DP-1".into(), tune());
            }
            for now in [moment(from), moment(to), moment(from), moment(to)] {
                show(&hub, now);
                let open = !now.covered && now.streaming;
                let expected: BTreeSet<String> = open.then(|| "DP-1".to_owned()).into_iter().collect();
                assert_eq!(real(&hub, Input::Media), expected, "media after {now:?} (from {from} to {to})");
                assert_eq!(
                    real(&hub, Input::Pointer),
                    expected,
                    "pointer after {now:?} (from {from} to {to})"
                );
                assert_eq!(*hub.tracking.borrow(), open || now.preview, "pointer feed after {now:?}");
            }
        }
    }
}
