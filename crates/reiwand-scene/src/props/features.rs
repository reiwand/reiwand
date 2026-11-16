use super::{Condition, Control, Mode, Property};

impl Property {
    pub fn features(&self) -> Vec<&'static str> {
        let control = match &self.control {
            Control::Slider(s) => s.step.is_none() || s.decimals.is_some() || s.editable || s.log,
            Control::Combo { options, default } => {
                !default.is_string() || options.iter().any(|o| !o.value().is_string() || o.icon().is_some())
            }
            Control::Text { secret, .. } => *secret,
            Control::File { default_asset, .. } => default_asset.is_some(),
            Control::Group { collapsed } => *collapsed,
            Control::Button {} | Control::Divider {} | Control::Shortcut {} | Control::Folder { .. } => true,
            _ => false,
        };
        let shown = !matches!(self.visible_if, Condition::Equal(_)) || !self.enabled_if.is_empty();
        let newer = control || shown || self.icon.is_some() || self.order.fract() != 0.0;
        let folder = matches!(self.control, Control::Folder { .. });
        let (lists, deeper) = match &self.control {
            Control::Folder { mode, max_files, recursive, .. } => {
                (*mode == Mode::All, *max_files != 1000 || *recursive > 0)
            }
            _ => (false, false),
        };
        let found = [
            (newer, "properties2"),
            (folder, "folder"),
            (self.accepts("video"), "user_videos"),
            (deeper, "folder2"),
            (lists, "folder_lists"),
            (matches!(self.control, Control::Shortcut {}), "shortcut"),
            (!self.quality.is_empty(), "quality_settings"),
        ];
        found.into_iter().filter(|(on, _)| *on).map(|(_, name)| name).collect()
    }
}
