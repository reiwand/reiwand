# Preset packages

The starting point `preset` (the *Preset* card of the *Scenes* page, `reiwand-ctl scene new DIR --from
preset`) creates a package of kind `preset` (`package.md`, "Presets"): the manifest, with the
entry `preset.json`, and `preset.json` itself, which the editor reads next to the manifest as the
document part `/preset` (`base`, `values` and `files`), with undo, redo and saving like the rest,
so `set`, `add` and `remove` reach it too. The *Package* page shows, for such a package, the
section *Preset* instead of *Required packages*: a box with the installed packages (or any id typed
by hand) and a version range to choose the base, the values and files already set, one row each
with a remove button, and a row to add a value (a setting id and its value, JSON or plain text such
as `#3d5aff` or `0.4`) or a picture or video that is copied into `assets/`. Title, license, origin
and the preview picture are the ones of any package (*Title, license, origin and more…* and *Use as
the thumbnail*). Everything goes through the `manifest` action `preset`, with `base` (`{id,
version}`, where a plain version such as `1.2.3` becomes its compatible range `^1.2` and a missing
one means any), `values` (setting id to value), `files` (setting id to the path of a picture or
video to copy) and `clear` (setting ids to forget): the base is also written in
`requires.packages`, a setting is in `values` or in `files` and never in both, a file that is not a
picture or a video is refused, and the package cannot be its own base. A preset without a valid
base is a problem of the package until one is chosen. Building, signing and publishing it are the
ones of any package (`reiwand-ctl pack build`, `pack sign`, `publish`); the values the installed base
does not know are left out with a warning when the preset is applied. `reiwand-ctl scene act DIR
manifest '{"preset": {"base": {"id": "org.example.dusk", "version": "1.2.3"}, "values": {"tint":
"#3d5aff"}}}'` does the same from a terminal.
