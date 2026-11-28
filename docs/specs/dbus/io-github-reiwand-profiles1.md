# `io.github.reiwand.Profiles1`

Profiles store a complete assignment of wallpapers to outputs, optionally with settings;
presets store a fit and a background color, for one item or for every item. Profile arguments
take a full uid or a name (names compare without case).

A profile with rules (`SetAuto`) is activated automatically when every rule it sets holds: after
the set of connected outputs has stayed the same for 2 seconds following a connect or
disconnect, and when the power source changes. The highest `priority` wins, ties go to the
first name. A manual `Activate` holds until the next such change; when nothing matches, nothing
changes. At start the rules wait for the power source (at most 5 s), so a battery state not yet
known does not end a hold. The choice also survives a restart of the daemon: the profile chosen
with `Activate` (or cleared with `""`) and the connected outputs and power source at that moment
are kept in `~/.local/share/reiwand/profile-hold.json` (`0600`) until the outputs or the
power source differ from them. A moment without any output, as while the daemon starts, does not
end the hold. A manual `Activate` and an automatic activation never overlap: an automatic one
that was about to start when the user chose a profile waits, then sees the hold and does nothing.

### `List() → (s json)`

A `ProfilesReply`.

### `Get(s profile) → (s json)`

A `ProfileReply`.

### `Save(s name, b with_options) → (s uid)`

Saves the current assignment of every output (its item, file or playlist, fit, background, preset
and filter), and with `with_options` every setting except
`models.allow_download` (which `Activate` never applies either), as a profile. An existing
profile of that name is replaced and keeps its uid, rules and priority.

### `Activate(s profile)`

Applies a profile at once and marks it active; `""` only clears the active marker. Outputs not in
the profile lose their own wallpaper. Entries whose item, file or playlist is gone or cannot be
shown are skipped with a warning, and those outputs show the default section; so are photosensitive
packages while `playback.allow_flashing` is off, and `GetState().outputs[x].error` says why until that
output is changed again. Unknown or invalid settings are skipped too. The configuration is saved
once and every output is reconfigured.

### `Rename(s profile, s name)`

Renames a profile.

### `Delete(s profile)`

Deletes a profile; deleting the active profile clears the marker.

### `SetAuto(s profile, s key, s value)`

Sets a rule for automatic activation: `outputs` (comma-separated connector names, met when
exactly these outputs are connected; `""` unsets), `on_battery` (`true`, `false`; `""` unsets)
or `priority` (`-1000` to `1000`).

### `ListPresets(s item) → (s json)`

A `PresetsReply`: the presets of `item`, or the global presets for `item = ""`. Presets of the
settings of a package (`SaveScenePreset`) are listed by `GetSceneProperties` instead.

### `SavePreset(s item, s name, a{ss} values) → (s uid)`

Saves a preset of `item` (`""` = global) with `fit` and/or `background` (`#rrggbb`). An
existing name in that scope is replaced and keeps its uid, and the configuration sections that
use it get its new fit and background.

### `DeletePreset(s preset)`

Deletes a preset; configuration sections that used it keep their fit and background.
