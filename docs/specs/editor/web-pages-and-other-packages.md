# Web pages and other packages

The editor opens and creates every kind of package folder, not only scenes. A folder whose
manifest has a `kind` other than `scene` or `assets` is a document of the manifest alone
(`{"manifest": …}`, without `scene`): actions on `/manifest` work as in a scene, saving writes only
`manifest.json`, and the problems are those of the manifest, the files of the folder (`validate`
of the format: a missing entry, a preload that is not in the folder) and the verdicts of each
platform. The outline adds `kind` and, for such packages, `files`: every file of the folder but
hidden ones, with `path`, `size`, `kind` (`page`, `script`, `style`, `wasm`, `data`, `image`,
`video`, `sound`, `font` or `other`) and `role` (`entry`, `preload`, `preview`, `manifest`,
`locale`, `signature` or `file`).

The starting point `web` writes `index.html` and `app.js`, a clock that takes its color from a
`color` property, hides nothing and stops its timer while the page is paused, and a valid `web`
manifest (`entry` `index.html`, no inputs, `web.background`). `reiwand-ctl scene open DIR` lists the
files of such a folder with their role, kind and size, and `--from FILE.wpak` extracts a package of
any kind into an empty folder.

| Action | Arguments | `reiwand-ctl scene act DIR manifest` |
|--------|-----------|-------------------------------|
| `manifest` `add_file` | `file`, `to` (a folder of the package, optional); copies the file into a web, picture or video package folder, never over `manifest.json` or its signature, and records nothing in the history | `add-file FILE [--to FOLDER]` |
| `manifest` `media` | `file`; copies a picture or a video into the folder of a `image` or `video` package, as its `entry` (it must be of the kind of the package, a video for a `video` package, and must pass the checks of the format); undo returns to the earlier entry | `scene act DIR manifest '{"media": {"file": "FILE"}}'` |

The starting points `image` and `video` write the manifest of a picture or video package (`kind`, an
`entry` of `picture.png` or `video.mp4` that is not in the folder yet, so the problems ask for the
file) and the *Scenes* page offers them next to the others, `reiwand-ctl scene new` as well.

The app opens such folders on a page of their own, *Package*, built as the workbench of the scene
editor (`PackageDocks`) in the Wallpaper layout, with the panels such a package has: See as (the
preview of the package, in the canvas's place), Files (size, role, *Add files…*, *Open the folder*,
and per file *Use as the entry page* or *Use as the thumbnail*), Options for a web page or Presets for
a video, Try, the Inspector, the Player, Problems, the Wallpaper sheet (whose first section,
"Package", holds the settings of the kind described below), Texts and languages, How it will look
and Review. The panels of a scene (Layers, Effects, Timeline, Curves, Screens and the canvas's See as)
are left out of its tree, and the groups, tabs, edges, the tab menu and dragging work as in the scene
editor. The six layouts are offered as in a scene (Ctrl+1 to Ctrl+9 included), each from the full
tree of that layout (`benchfull.rs`, every panel in view) without the panels of a scene, so a
package page shows its files, inspector and Try at once. A package opens in Wallpaper every time, whatever layout
was chosen on the page before. Each kind keeps its own arrangement of
every layout, and the layouts the author saves on its pages, in a file of its own next to the
scene's: `workbench-web.json`, `workbench-image.json`, `workbench-video.json` and
`workbench-preset.json`, in the same format and read with the same rules as `workbench.json`. A
missing file is the six layouts as they come, each without the panels of a scene; what is moved on a
package page never changes the layouts the scenes use, nor those of another kind.

For a web page the settings of the kind start with *Page*: a list of the scripts of the folder to
run before the page, with *Add a script…* to copy a `.js` or `.mjs` file in and choose it at once
(`web.preload`, the `web` action with `preload`), a color picker for the color shown while the page
loads (`web.background`, `#rrggbb`) and a slider from 0 to 5120 KiB for the data the page keeps
(`web.storage_kib`, 0 keeps nothing). Each control sends the `manifest` action `web` with `values`
and `clear`, so `reiwand-ctl scene act DIR manifest '{"web": {"values": {"storage_kib": 256}}}'` does the
same and so does `reiwand-ctl scene act DIR manifest`; a color that is not `#rrggbb`, a size over
5120 KiB or a preload that is not in the folder is refused like any other invalid manifest.

The *See as* panel shows the page as it is in the folder, with the manifest as it
is in the editor, inside the window: the preview stream of `PreviewScene` (`dbus.md`) is made once
the panel is first shown and covered, so the page pauses, while another tab of its group is
open. *Reload* builds the
page again from its files, which is how a change made in another editor shows; the quality
selector (Q0 to Q3) runs the page at that level of the output and the frame rate selector caps its
frames (15, 30, 60 or not capped), both of the preview only. What the page writes with `console.log`
and the errors it raises appear in the console under the problems. `reiwand-ctl scene preview DIR`
does the same on the desktop: for a web page it sends the manifest and the staged build, with
`--quality`, `--restart` (reload) and `--console`.

Under the preview of a web page, *The page and its lights* shows the colors the page sends with
`window.reiwand.rgb.setImage` as a grid, read ten times a second from `GetStrip` of the preview output while
the tab is shown, and a switch that sets or removes `permissions.rgb` in the manifest (the same
`set` and `remove` actions on `/manifest/permissions/rgb`). A preview lights the strip without the
grant, with the lights off and without sending anything to the devices, so the author sees what a
user would get before publishing; `reiwand-ctl rgb strip -o OUTPUT` reads the same strip for the output of
`reiwand-ctl scene preview DIR -o OUTPUT`.

The *Options* panel of a web page (`PackageDocks.qml`, in the Wallpaper layout) is the list of user properties of the scene editor, the same
widget and the same actions (`add_property`, `set`, `remove`, the property inspector on the right):
sliders, switches, colors, choices, texts, buttons, groups and, for a page, the `file` and `folder`
properties whose choice the page reads through `window.reiwand.files` and `window.reiwand.random`; the
user picks a folder per screen in the settings of the page. `reiwand-ctl scene act DIR add` and the other property
commands do it from a terminal, and so does `reiwand-ctl scene act`.

*Network* lists the https origins the page may reach, one row each with a remove button, and a field
with *Add origin* (at most 8). It sends the `manifest` action `network` (`origins`, with `add` or
`remove` to change the list instead of replacing it): origins are written in lower case without a
trailing slash, a repeated one is kept once, and `network` is added to `inputs` with the first origin
and removed with the last, so the declared input and the origins never disagree; `inputs` refuses to
add or remove `network` by itself. Only a web page takes origins. The same action is
`reiwand-ctl scene act DIR manifest`.

A preset package (*Preset* in the settings) chooses its base from the installed packages and then lists
the real settings of that base, in its order, each with the control of the property itself: a color
picker for a color, a slider with the range and unit of a slider, a switch, a box with the labelled
choices of a combo, a field for a text, and *Picture or video…* for each `file` property (the file is
copied into `assets/`). A setting the preset sets is shown in bold and *Default* takes it out; the
others show the default of the base. Every control sends the `manifest` action `preset` (`values` or
`files`, `clear`), so `reiwand-ctl scene act DIR manifest '{"preset": {"values": {"speed": 0.4}}}'`
does the same. The list comes from the `base` query of `rig`, `{"base": {"file": "<package or
folder>"}}`, which reads the manifest of the base package, and answers `{id, version, fields, files}`:
the fields are those of the inspector (`widget`, `range`, `unit`, `options`, `names`, `default`,
`value` and `set` from the values of the preset, with `/preset/values/<id>` as pointer) and `files`
the `file` properties with their `accept` and the picture the preset gives. A base that is not
installed shows no settings and asks to install it.

A picture or video package shows, in the "Package" section of its Wallpaper sheet: the file (*Choose the picture…* or *Choose the
video…*, the `media` action, with its size and length), *Display* and, for a video, *Playback*.
*Display* sends the `display` action: the placement `fit` (fill, fit, stretch, center, tile or
span), the mirror `flip` as three buttons, the `background` color, `span` (*Spread over every
screen*) and, for a video, `zoom` (1 to 8) and `position` with an alignment pad: a rectangle with a
handle that is dragged (or clicked) to choose which part of the zoomed video shows, from -1 to 1 on
each side, with magnets at the edges and the middle and a button that centers it again. *Playback*
of a video starts with the trim on a timeline over the real length of the video (the `media` query of
`rig`, `{"media": {}}`, answers `{kind, entry, present, bytes, length, size, aspect}`; `length` comes
from the file, a picture gives its `size` and a video its `aspect`): a strip of thumbnails (the cache
of `Strips`, also used by the clocks of the timeline) with a bar for the part that plays and two grips
that set `start` and `end` of the `playback` action when they are let go (an `end` dragged to the end
of the video is taken out, the grips never cross) and *Whole video*; a speed slider from 0.1 to 4
(logarithmic) and *Loop*; and the volume, muting and frame rate that web pages have. A video package
calls that panel *Presets*: *Add preset* adds `/manifest/presets/N`, and the
inspector shows one row per video setting (speed, start, stop at, volume, zoom, x and y) with the
slider and the range of the video itself (`start` and `end` up to its length), so `set` on
`/manifest/presets/N/values/<id>` and `scene act` do the same.

The *Preview* tab also exists for pictures and videos: the editor sends the daemon a scene made from
the folder (`reiwand_editor::reels::preview_document`): one layer that shows the file at the proportion
of the picture or video, a canvas that places it with `display.fit` and `display.background`, the
mirror and the zoom as the scale and the part shown as the position, and `playback` as the playback
of the layer (speed, start, end and loop) and of the manifest (frame rate, volume and muting). The
quality selector and the frame rate selector are the same as the ones of a web page.

*Required packages* (in the "Package" section of the Wallpaper sheet of every kind of package) lists `requires.packages`, one row
each with a remove button, and a picker to add one: a box that lists the installed packages of the
library (assets packages included, never the package itself) and also takes an id typed by hand, and
a version range. Left empty, the range is the one compatible with the installed version, `^1.2` for
`1.2.7`. It sends the `manifest` action `requires` (`packages`: `{id, version}`, with `add` or
`remove` to change the list instead of replacing it). A `version` that is a plain version, such as
`1.2.7` or `0.4.1`, becomes its compatible range (`^1.2`, `^0.4`, `^0.0.3` for `0.0.3`) and any
other text must be a SemVer requirement, otherwise the action is refused; an id already listed
takes the new range, the package cannot require itself, an id must be valid, at most 16 packages
fit, and `requires.features` follows (`packages` with the first one, removed with the last).
`reiwand-ctl scene act DIR manifest '{"requires": {"packages": [{"id": "org.example.fx", "version":
"1.2.7"}], "add": true}}'` does the same from a terminal.

*Inputs* has a check box for each input a page cannot show it uses, since nothing in its code tells
the tools: `audio`, `pointer`, `clicks` and `media`; the `network` box is read-only and follows the
origins. A box sends the `manifest` action `inputs` with `add` or `remove`, which refuses a name that
is not an input. The same action is `reiwand-ctl scene act DIR manifest`.

*Playback* sets how the page plays by default: the highest frame rate (not set, follow the
screen, or 10 to 240 frames a second), the volume as a slider with *Default* to remove it, and
whether it starts without sound. It sends the `manifest` action `playback` with `values` and
`clear`; `speed`, `start`, `end` and `loop` are only for videos and a page refuses them. What the
user sets for a screen (frame rate, volume, muting) always wins over these defaults. The same
action is `reiwand-ctl scene act DIR manifest`.
