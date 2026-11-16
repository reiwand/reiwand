use super::*;
use reiwand_core::model::InputsGate;
use std::collections::BTreeMap;

const VIEW: View = View { active: true, covered: false, unseen: false, rect: None, clicks: false };

#[test]
fn each_screen_says_why_its_inputs_are_off() {
    let hub = hub(Arc::default());
    let views = BTreeMap::from([
        ("DP-1".to_owned(), VIEW),
        ("DP-2".to_owned(), View { covered: true, active: false, ..VIEW }),
        ("DP-3".to_owned(), View { unseen: true, ..VIEW }),
        ("DP-4".to_owned(), View { active: false, ..VIEW }),
        ("preview:1".to_owned(), VIEW),
    ]);
    hub.update(Some((views, false)));
    let gates = crate::inputs::gates(&lock(&hub.control));
    let expected = BTreeMap::from([
        ("DP-1".to_owned(), InputsGate::On),
        ("DP-2".to_owned(), InputsGate::Covered),
        ("DP-3".to_owned(), InputsGate::Unseen),
        ("DP-4".to_owned(), InputsGate::Still),
    ]);
    assert_eq!(gates, expected, "previews are not screens and each gate names its reason");
}
