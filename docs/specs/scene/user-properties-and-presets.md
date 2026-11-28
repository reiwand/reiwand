# User properties and presets

User properties (settings) are declared in the manifest, under `properties` (id → definition), and
scenes refer to them by id. Each has a `type` and may set `label`, `help`, `group`, `visible_if`,
`enabled_if`, `order`, `icon` and, for sliders, combos and flags, `quality`:

| `type` | Fields | Value |
|--------|--------|-------|
| `color` | `default`, `alpha` | `#rrggbb` or `#rrggbbaa` |
| `slider` | `min`, `max`, `default`, `step`, `decimals` (0 to 6), `integer`, `unit`, `editable`, `log` | number, clamped to the range, aligned to the step and rounded to the decimals |
| `bool` | `default` | `true` or `false` |
| `combo` | `options` (values or `{ "value", "label", "icon" }`, at most 64), `default` | one of the option values: texts, numbers or `true` and `false`, all of one kind |
| `text` | `default`, `max_len` (≤ 1024), `multiline`, `secret` | text |
| `file` | `accept` (`image`, `video`), `default_asset` | absolute path of a file chosen by the user, or `null` |
| `folder` | `accept`, `pick` (`random`, `sequence`), `every` (10 to 86400 s), `mode` (`one`, `all`), `max_files` (1 to 1000, prefilled 1000), `recursive` (0 to 2 levels of subfolders, prefilled 0) | absolute path of a folder chosen by the user, or `null` |
| `shortcut` | | what the user chose to open: `{"uri": "https://…"}`, `{"file": "/…"}`, `{"folder": "/…"}` or `{"app": "<desktop id>"}`, or `null` |
| `button` | | none; pressing it is an event |
| `group` | `collapsed` | none; the properties that name it are listed under it |
| `label`, `divider` | | none; a line of text, or a line across the form with an optional label (a divider without one shows no text) |

A slider without `step` moves in steps of its decimals, which come from `decimals`, else from
`step` and the decimals of `min` and `max` (the most of the three), else two (none for `integer`);
its value is always rounded to them. `min` and `max` are always accepted as they are, even when
they are not on the grid of `step` (`max` 0.175 with `step` 0.01 keeps 0.175), and no rounding
takes a value out of the range. `editable` adds a box to
type the number and `log` spreads the range logarithmically (it needs `min > 0`). `order` is any
number: properties sort by it and then by id, and the members of a group follow it in their own
order; a group cannot be in another group. An option of a combo with an empty label shows its value.
On the phone a `color` opens a mixer with red, green and blue sliders (and opacity with `alpha`)
and a box for the hex code; a `multiline` text is a box of several lines saved with its own button,
and `max_len` stops the typing at that length and counts the characters typed against it.
`icon` is one of `appointment-soon`, `audio-volume-high`, `audio-volume-muted`, `audio-x-generic`,
`camera-photo`, `dialog-information`, `document-open`, `edit-find`, `emblem-favorite`, `folder`,
`go-home`, `image-x-generic`, `input-keyboard`, `input-mouse`, `media-playback-start`,
`media-playlist-repeat`, `media-playlist-shuffle`, `preferences-desktop-theme`,
`preferences-system`, `text-x-generic`, `video-display`, `video-x-generic`, `view-refresh`,
`weather-clear`, `weather-clear-night`, `weather-few-clouds`, `weather-overcast`, `weather-showers`,
`weather-snow`, `weather-storm`, `x-office-calendar` or `zoom-in`, symbolic icons of the desktop
theme; other names of `[a-z0-9-]` are accepted and not shown, so that newer versions can add icons.
Packages never bring their own icons.

`visible_if` hides a property and `enabled_if` greys it out while a condition does not hold, and a
group's conditions apply to its members. A condition is a map `{ "<other id>": value }` (all must
match) or an expression of a small language, never code: property ids, numbers, texts in `'…'` or
`"…"`, `true`, `false`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!` and parentheses, such as
`"mood == 2 || speed > 1.5"`. An expression has at most 256 characters and 8 levels of nesting;
it may compare a property only with a value of the same kind (`<` and the like only numbers), and
the validator refuses unknown ids, properties without a value, conditions that are not true or
false and conditions that depend on themselves through other properties.

A `button` holds no value: pressing it (`PressSceneButton`, `reiwand-ctl prop press`) starts the
timelines whose `trigger` is `prop:<id>` and shows the press to the scripts for one frame, and it
cannot be pressed while its `enabled_if` or that of its group does not hold. Only a property that
holds a value may be followed with `{ "prop" }`, and a `trigger` `prop:<id>` names a button or such
a property. A `secret` text has no default and is kept in the Secret Service on the desktop and
sealed by the Android keystore on the phone, never in `config.toml`, logs, presets, exported
settings or sync; a scene reads it as an empty text, since only content allowed to use the network
may receive it.

A `file` chosen by the user replaces the asset named by `default_asset` wherever the scene uses it
(image and video layers, masks, effect textures and particle textures), and also shows wherever
`{ "prop": "<id>" }` names the property, with or without `default_asset`. Images are decoded like
the scene's own; with `accept: ["video"]` a video plays in image and video layers without sound,
in a loop and paused with the scene, while masks, effect textures and particles keep the package's
own asset. Image and video layers, the `texture` parameters of effects, the `texture` of masks and
the `render.texture` of particles take `{ "prop", "map" }`, so that a combo or a flag switches
between assets of the package (variants), and `{ "prop": "<id>" }` of a `file`; `reiwand-ctl pack
check` warns when the images of the variants of one place do not share one proportion. A new
choice replaces only the textures it touches: the clock, the timelines, the particles (unless
their own texture changed) and the scripts of the scene go on. A `folder` shows one of its
images (or videos, with `accept`) in an image layer bound to it with `{ "prop": "<id>" }`, picked
at random or in order and changed every `every` seconds while the scene is shown; only the folder
itself is read (and `recursive` levels of its subfolders), without following links out of it or
reading hidden files, at most `max_files` of them in name order, and nothing is read while the scene is not shown, paused, covered or behind the lock screen: a
turn missed then happens once when the scene shows again. Files and folders are chosen by the user
through the file chooser, are never set by presets and never travel with exported settings or sync.
`mode: all` lets web pages list every file of the folder (`files`, `onFiles`, `web.md`), where
`one` only gives them one file at a time (`randomFile`); scene scripts list it with `folder(prop)`
(see "Scripts"), which needs the feature `script_folders`.

A `shortcut` holds what the user chose to open from the wallpaper: an `https://` address of at
most 2048 characters (opened through the OpenURI portal of the desktop, which may ask first), a
file or a folder (chosen through the file chooser and opened through the same portal, which gets
it as an open file) or an application by its desktop id (`[A-Za-z0-9._-]`, at most 255 characters),
never a command. On the phone a file or a folder is the one the user picked with the system document picker, kept as a `content://` address with read permission that survives restarts (`{"file": "content://…"}` or `{"folder": "content://…"}`), and it is opened through the same view intent as the other targets. Only the user sets it (`SetSceneProperty`, `reiwand-ctl prop set open
'{"uri": "https://example.org"}'`, or the app's control under *Scene settings*), never the author, a
preset or sync, and `SetSceneProperty` refuses a file or folder that does not exist, an application that is not installed
and an address that is not one (`CheckShortcut` asks the same without storing it, and `ListApplications`
lists the installed applications the app offers as a picker; `reiwand-ctl prop apps` and `prop
check-shortcut` do the same). It stays in
`config.toml` on this computer. An application is looked up as `applications/<id>.desktop` in the
data folders of the desktop: one that is hidden or runs in a terminal is refused, a `DBusActivatable`
one is activated through `org.freedesktop.Application`, and any other is started from its `Exec`
line without a shell and without its field codes; applications open only outside the Flatpak. The
content never sees the target, only `true` when one is set and `null` otherwise: a script calls `openShortcut(prop)` inside the handler of a real
`click` of a layer and a page calls `reiwand.openShortcut(prop)`; the daemon opens it only within 2 s
of a click that reached the scene or page on a screen that takes clicks, for a package that holds
the `clicks` grant, at most once a second per screen, and only for a `shortcut` property of the
package shown there. On the phone the user chooses an installed app (one the launcher lists, by its
package name) or an `https://` address from the page of the package (files and folders stay on the
desktop), and the app opens it with Android only within 2 s of a tap on the home screen, for a
package that holds the taps grant, at most once a second, never from the lock screen or a preview. `quality` maps
quality levels (`q0` to `q3`, `q3` the highest) to a value of the property: on that level and the
lower ones the value is at most that one for a slider and exactly that one for a combo or a flag,
as the scene, its scripts and web pages see it (the stored value does not change), so a
power-saving mode turns itself on
with the battery.

`presets` is a list of `{ "id", "label", "values" }`; a preset cannot set files, folders, secrets
or buttons. Values are changed through `GetSceneProperties`, `SetSceneProperty`,
`ApplyScenePreset` and `ResetSceneProperties` (`dbus.md`) or `reiwand-ctl prop`, and stored in
`config.toml`, once for every screen or per screen: a screen without values of its own follows
and changes the shared ones until it gets its own copy (`ShareSceneValues`, `reiwand-ctl prop share`,
*Same settings on every screen* in the app), which it keeps even when emptied by a reset. The
user can also save the current values as a preset of their own, rename, delete, export it to a
small file and import it back (`{"format": "wpak-values/2", "package", "version", "values"}`); saved
presets belong to the package id, survive updates and travel with sync. A stored value whose
property no longer exists, changed its type or falls outside its range after an update of the
package is ignored and the property uses its own `default`, with a warning in the log, in `GetSceneProperties`
(`warnings`) and in `reiwand-ctl prop ls`. Labels, help, option labels and preset labels are text or
`{ "<language>": text }` or `"@key"` of the manifest's `strings`; the newer kinds and fields above
need the feature `properties2` (`folder` and `user_videos` for folders and
videos chosen by the user, `folder2` for `max_files` and `recursive`, `folder_lists` for
`mode: all` in web pages and `script_folders` in scenes, `shortcut` and `quality_settings`; `package.md` says which platforms run each). The app shows these forms under *Scene settings* for each screen on its
Settings page, and the Android app on the page of the scene. The scenes in the `scenes/` folder of
the repository are examples of this format, dedicated to the public domain.
