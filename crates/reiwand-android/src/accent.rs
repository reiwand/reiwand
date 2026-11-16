use reiwand_format::Limits;
use reiwand_library::{Item, Kind};

pub(crate) fn accent(item: &Item) -> Option<i32> {
    if item.kind != Kind::Package {
        return None;
    }
    let package = reiwand_format::open(&item.path, &Limits::default()).ok()?;
    let accent = package.manifest.display.as_ref()?.accent.clone()?;
    let values = match accent {
        reiwand_format::Accent::Prop { .. } => crate::props::values(&*crate::pack::load(&item.path).ok()?),
        reiwand_format::Accent::Color(_) => Default::default(),
    };
    accent.resolve(&values).map(|rgb| (0xff00_0000 | rgb) as i32)
}
