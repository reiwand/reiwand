use reiwand_core::rgb::{Flow, Place, Rgb};
use reiwand_rgb::{Area, Device};

fn flow(flow: Flow) -> reiwand_rgb::Flow {
    match flow {
        Flow::Right => reiwand_rgb::Flow::Right,
        Flow::Left => reiwand_rgb::Flow::Left,
        Flow::Down => reiwand_rgb::Flow::Down,
        Flow::Up => reiwand_rgb::Flow::Up,
    }
}

fn placed(place: Place) -> Option<Area> {
    match place {
        Place::Off => None,
        Place::At { rect, flow: f } => Some(Area { rect, flow: flow(f) }),
    }
}

pub(super) fn area(rgb: &Rgb, device: &Device) -> Area {
    rgb.place(&device.key).and_then(placed).unwrap_or_default()
}

pub(super) fn zone(rgb: &Rgb, device: &Device, zone: usize) -> Option<Option<Area>> {
    rgb.zone_place(&device.key, zone).map(placed)
}
