# `io.github.reiwand.Daemon1`: `ListApplications →` to `ShareSceneValues`

On this page: `listapplications →`, `checkshortcut`, `applyscenepreset`, `resetsceneproperties`, `savescenepreset →`, `renamescenepreset`, `deletescenepreset`, `exportscenevalues →`, `importscenevalues →`, `pressscenebutton`, `capturepage →`, `getparticlestats →`, `sharescenevalues`.

## `ListApplications() → (s json)`

An `ApplicationsReply`: `available` (`false` in the Flatpak, where applications are not opened) and
`applications`, the applications a `shortcut` setting can open, each `{id, name, icon}`: the
desktop entries of `applications/` in the data folders of the desktop (`[A-Za-z0-9._-]` ids, the
first of an id wins) that are of type application and are neither hidden, nor `NoDisplay`, nor run
in a terminal, ordered by name. `icon` is the `Icon` of the entry, a theme name or a path, and may
be empty. The app offers them as a picker and `reiwand-ctl prop apps` lists them.

## `CheckShortcut(s value_json)`

Checks the target of a `shortcut` setting without storing it, with the rules of `SetSceneProperty`:
`null` (nothing), `{"uri": "https://…"}` (an address with a host, at most 2048 characters, without
spaces or control characters), `{"file": "/…"}` or `{"folder": "/…"}` (absolute, existing) or
`{"app": "<id>"}` (installed and launchable, outside the Flatpak). Anything else returns
`InvalidArgs` with the reason, which the app shows as the target is typed. `reiwand-ctl prop
check-shortcut VALUE` does the same.

## `ApplyScenePreset(s output, s preset)`

Stores every value of a preset as `SetSceneProperty` would: a preset of the package by its id or its
label, or a saved preset by its uid or name, ignoring case. Files, folders and secrets are never
part of a preset. `NotFound` for a preset that does not exist.

## `ResetSceneProperties(s output)`

Forgets the values stored for the package shown on `output` (`""` = the `[default]` section): the
screen's own values when it has them, which it keeps having (empty), or else the shared ones, so its
settings take their defaults. Secrets stay stored.

## `SaveScenePreset(s output, s name) → (s preset)`

Saves the current values of the package shown on `output` as a preset named `name` and returns its
uid; saving a name the package already has replaces that preset and keeps its uid. Saved presets
belong to the package id, so they survive updates of the package, live in the library.

## `RenameScenePreset(s preset, s name)`

Renames a saved preset. `Exists` when the package already has a preset of that name.

## `DeleteScenePreset(s preset)`

Deletes a saved preset.

## `ExportSceneValues(s output, s preset) → (s values)`

The current values of the package shown on `output`, or those of a preset when `preset` is not
`""`, as a small document `{"format": "wpak-values/2", "package", "version", "values"}` that the
user may keep, copy or share. Files, folders and secrets are left out.

## `ImportSceneValues(s output, s json, s name) → (as warnings)`

Reads a `wpak-values/2` document of the package shown on `output` (at most 256 KiB): with
`name = ""` its values are stored as `SetSceneProperty` would, otherwise they are saved as a preset
named `name`. A document of another package returns `InvalidArgs`; settings that the package does
not have or no longer accepts, files, folders and secrets are skipped and listed in `warnings`.

## `PressSceneButton(s output, s id)`

Presses the `button` setting `id` of the scene shown on `output` (`""` = every output that shows
it): timelines whose `trigger` is `prop:<id>` start again and the scripts of the scene see the press
in their next frame (`scene.md`). Nothing is stored.

## `CapturePage(s output) → (s path)`

Takes one frame of the web page that `output` shows, for the editor of web packages (`reiwand-ctl
scene thumbnail DIR --capture`, *Capture the thumbnail*). The page is the one that is shown,
started with the same helper, sandbox, network rules and settings as when it is a wallpaper: the
daemon asks the helper for a copy of the last frame it drew (`Shot`), and the helper sends it as
pixels. The first call asks for the frame and answers `""`; a later call, a moment after, answers
the path of a PNG file of that frame (mode `0600`, in the runtime folder, replaced by the next
capture). A client that gets `""` calls again after 200 ms, for at most 10 s. `NotFound` when
`output` shows no web page.

## `GetParticleStats(s output) → (s json)`

Counts the particles of the scene that `output` draws now, for the editor of scenes and
`reiwand-ctl prop particles --output`. The reply is `{ output, path, reason, alive, slots, systems,
quality, quality_reason, tier, tier_measured }`: `path` is `"gpu"` or `"cpu"`, the place that
simulates the particles, and `reason` why (`"compute"`: the card runs compute shaders,
`"software"`: the renderer is software, `"no_compute"`: the card lacks them, `"chosen"`);
`alive` counts the live particles read back from the simulation, `slots` those the scene holds at
this tier and `systems` lists `{ id, alive }` for every system; `quality` and `quality_reason` are
those of the output in `GetState`. Nothing is counted until it is asked, so a scene that nobody
asks about does no extra work. `NotFound` when the output does not exist or draws no particles.

## `ShareSceneValues(s output, b shared)`

With `shared`, the screen `output` stops having its own values for the package it shows: they move
to the `[default]` section, which every screen without its own values follows. Without it, the
screen gets its own copy of the shared values, which later changes on it keep for itself.
`InvalidArgs` for `output = ""`.
