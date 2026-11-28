use super::{Control, View};
use reiwand_core::model::{Input, InputsGate};
use std::collections::BTreeMap;

impl View {
    pub(super) fn place(&self, x: f64, y: f64) -> Option<[f32; 2]> {
        let [left, top, width, height] = self.rect.filter(|_| self.open())?;
        let at = [(x - left) / width, (y - top) / height];
        at.iter().all(|v| (0.0..1.0).contains(v)).then(|| at.map(|v| v as f32))
    }

    pub(crate) fn open(&self) -> bool {
        self.gate() == InputsGate::On
    }

    pub(crate) fn gate(&self) -> InputsGate {
        match (self.covered, self.unseen, self.active) {
            (true, ..) => InputsGate::Covered,
            (_, true, _) => InputsGate::Unseen,
            (_, _, false) => InputsGate::Still,
            _ => InputsGate::On,
        }
    }
}

pub(crate) fn reason(gate: InputsGate) -> &'static str {
    match gate {
        InputsGate::On => "in view",
        InputsGate::Still => "the wallpaper is paused",
        InputsGate::Covered => "the desktop reported a window covering the screen",
        InputsGate::Unseen => "the screen's stream is not playing",
    }
}

fn real(name: &str) -> bool {
    !crate::drafts::virtual_output(name) && !name.starts_with(crate::previews::PREVIEW)
}

pub(crate) fn gates(control: &Control) -> BTreeMap<String, InputsGate> {
    let views = control.views.iter().filter(|(name, _)| real(name));
    views.map(|(name, v)| (name.clone(), v.gate())).collect()
}

pub(super) fn told(control: &Control, views: &BTreeMap<String, View>) {
    let was = gates(control);
    let used = |name: &str| control.shown.contains_key(name) || control.parallax.contains(name);
    for (name, view) in views {
        let gate = view.gate();
        if !real(name) || was.get(name) == Some(&gate) {
            continue;
        }
        let wanted = used(name) && Input::ALL.iter().any(|i| control.allows(name, *i));
        tracing::info!(
            output = %name,
            active = view.active,
            covered = view.covered,
            streaming = !view.unseen,
            inputs = if gate == InputsGate::On { "on" } else { "off" },
            used = wanted,
            reason = reason(gate),
            "screen inputs"
        );
    }
}
