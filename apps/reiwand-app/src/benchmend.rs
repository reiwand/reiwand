use crate::benchdesk::{Chosen, Desk, Own};
use crate::benchkits::{KITS, Kit, View};
use crate::tr::tr;
use reiwand_core::message::render;
use serde_json::{Value, json};

/// The panels formats 3 to 5 added, closed in every view of a file written before them.
pub const ADDED: [(u32, &[&str]); 3] = [
    (3, &["effects", "curves"]),
    (4, &["viewas", "inputs", "screens"]),
    (5, &["sheet", "texts", "cover", "review"]),
];

/// Brings a file of the first two formats to the third: a single tree (format 1, and with the
/// panels it closed from format 2) becomes the Scene layout, the five others as they come.
pub fn older(value: &mut Value) {
    if value.get("format").and_then(Value::as_u64) == Some(1) {
        if let Some(content) = value.get_mut("content").and_then(Value::as_object_mut) {
            content.entry("closed").or_insert_with(|| json!([]));
        }
        value["format"] = Value::from(2);
    }
    if value.get("format").and_then(Value::as_u64) == Some(2) {
        let mut desk = serde_json::to_value(Desk::default()).unwrap_or_default();
        desk["factory"]["scene"] = value["content"].take();
        value["content"] = desk;
        value["format"] = Value::from(3);
    }
}

/// The layouts of a file that does not read whole, each read on its own: one that is broken goes
/// back to how it comes (or, for one of the author's, is left out) and the others are kept. None
/// when the file is not a set of layouts at all; else the desk and what was lost, each name with
/// whether it is one of the author's.
pub fn mend(value: &Value) -> Option<(Desk, Vec<(bool, String)>)> {
    let content = value.get("content")?.as_object()?;
    let mut desk = Desk::default();
    let mut lost = Vec::new();
    for kit in KITS {
        let name = serde_json::to_value(kit).ok()?.as_str()?.to_owned();
        let read = content.get("factory").and_then(|f| f.get(&name)).cloned();
        match read.and_then(|v| serde_json::from_value::<View>(v).ok()).filter(|v| v.fault().is_none()) {
            Some(view) => *desk.factory.get_mut(kit) = view,
            None => lost.push((false, word(kit))),
        }
    }
    for item in content.get("own").and_then(Value::as_array).into_iter().flatten() {
        let own = serde_json::from_value::<Own>(item.clone()).ok().filter(|own| {
            own.saved.fault().is_none()
                && own.view.fault().is_none()
                && desk.name_fault(&own.name, None).is_none()
        });
        match own {
            Some(own) => desk.own.push(own),
            None => lost.push((true, item.get("name").and_then(Value::as_str).unwrap_or("?").to_owned())),
        }
    }
    let chosen = content.get("chosen").and_then(|c| serde_json::from_value::<Chosen>(c.clone()).ok());
    desk.chosen = match chosen {
        Some(Chosen::Own(name)) if !desk.own.iter().any(|own| own.name == name) => {
            Chosen::Factory(Kit::Scene)
        }
        Some(chosen) => chosen,
        None => Chosen::Factory(Kit::Scene),
    };
    desk.fault().is_none().then_some((desk, lost))
}

/// What the editor says of a file it mended.
pub fn told(file: &str, lost: &[(bool, String)]) -> String {
    let names =
        |own: bool| lost.iter().filter(|l| l.0 == own).map(|l| l.1.as_str()).collect::<Vec<_>>().join(", ");
    let mut said = render(&tr("Part of %1 could not be read."), &[file.to_owned()]);
    for (own, words) in [
        (false, tr("These layouts went back to how they come: %1.")),
        (true, tr("These layouts of yours were left out: %1.")),
    ] {
        if lost.iter().any(|l| l.0 == own) {
            said += &format!(" {}", render(&words, &[names(own)]));
        }
    }
    said + " " + &tr("Your other layouts are kept.")
}

fn word(kit: Kit) -> String {
    tr(match kit {
        Kit::Scene => "Scene",
        Kit::Particles => "Particles",
        Kit::Scripts => "Scripts",
        Kit::Simulation => "Simulation",
        Kit::Animation => "Animation",
        Kit::Wallpaper => "Wallpaper",
    })
}
