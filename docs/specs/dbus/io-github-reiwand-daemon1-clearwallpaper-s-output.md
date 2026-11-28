# `io.github.reiwand.Daemon1`: `ClearWallpaper` to `SetSceneProperty`

On this page: `clearwallpaper`, `restorewallpaper`, `setlockwallpaper`, `setlockplaylist`, `followdesktoponlock`, `showdesktoponlock`, `getlock →`, `handbacktoplasma →`, `setbackground`, `setfilter`, `getsceneproperties →`, `setsceneproperty`.

## `ClearWallpaper(s output)`

Removes the wallpaper of `output`, or of every output when `output` is empty, and gives the screen
back to the desktop. A playlist assigned to the affected outputs stops. What was shown is kept in
the configuration (`blank` of the section) until a new wallpaper is set;
`GetState().outputs[x].blank` is true while an output shows no wallpaper and `previous` while
`RestoreWallpaper` has something to bring back. Clearing again keeps the first wallpaper, not the
empty one. What the screen shows then depends on how the daemon presents:

- Layer-shell surfaces: the daemon destroys the layer surface of the output, so the compositor
  shows whatever the desktop itself has there (its own background). No frames, timers or GPU work
  remain for that output, and `GetState().outputs[x].released` is true. The surface is created
  again when a wallpaper is set or restored.
- X11 windows: the daemon destroys the desktop window of the monitor and gives the root window
  back what it showed before the daemon first drew: the pixmap that `_XROOTPMAP_ID` and
  `ESETROOT_PMAP_ID` named, or, when the root window had none, a copy of what the root window showed
  at that moment (without other programs' windows). X cannot report a plain root color, so a color
  set with `xsetroot` comes back as that copy. If another program changed the root background in
  the meantime, it is left alone. `released` is true as for layer-shell.
- Plasma (the daemon draws through its wallpaper plugin): the plugin cannot switch itself away, so
  the screen shows the background color (`SetBackground`, black by default). `GetState().plasma` is
  true in this case. `HandBackToPlasma` switches to Plasma's own wallpaper type when the person asks.
- Streams for another program: a stream of the background color.

On a first start, with nothing chosen yet, the daemon shows nothing: layer-shell and X11 outputs
are listed in `GetState` with `released` and `blank` true but get no surface, so the desktop's own
background stays. In Plasma the plugin only runs where the person chose it in the desktop settings;
the daemon never changes the wallpaper type by itself.

Removing from the library an item that is the current wallpaper (`Library1.Remove`) clears the
outputs that showed it in the same way, and `RestoreWallpaper` brings it back while its file still
exists (`the earlier wallpaper's file is no longer there, so it cannot be brought back` otherwise).

## `RestoreWallpaper(s output)`

Brings back what `ClearWallpaper` removed from `output` (every output when empty). With nothing to
bring back it returns `InvalidArgs` (`there is no earlier wallpaper to bring back`) and changes
nothing. Layer-shell surfaces and X11 windows are created again.

## `SetLockWallpaper(s uid, s output)`

Shows the library item `uid` (a picture, a video, a scene or a page) on the lock screen of `output`,
or of every screen when `output` is empty, which also drops the choices of single screens. By
default the lock screen follows the desktop. The item is checked as in `Library1.Apply` (missing
file, flashing, revoked, assets) and a preset returns `InvalidArgs` (`a preset changes the settings
of a wallpaper on a screen; choose its base`). A name starting with `lock:` or `preview:` returns
`InvalidArgs`. The choice is kept in the `[lock]` block of `config.toml` and replaces a playlist
chosen there. Removing the item from the library drops the choice, as it clears the screens that
showed it.

## `SetLockPlaylist(s playlist, s output)`

Plays the playlist (uid or name) on the lock screen of `output`, or of every screen when `output` is
empty, with its own turns while the screen is locked: its rules apply as on a screen, and its
rotation is kept under the stream name (`lock:DP-1`). Deleting the playlist drops the choice;
stopping the desktop's playlist (`Playlists1.Stop`) does not.

## `FollowDesktopOnLock(s output)`

Drops the lock choice of `output`: that lock screen then takes the choice for every screen, or the
desktop when there is none. With an empty `output` every lock screen follows the desktop again and
the `[lock]` block leaves `config.toml`.

## `ShowDesktopOnLock(s output)`

The lock screen of `output` shows what its desktop shows even while every lock screen has a choice
(`desktop = true` in its table of `[lock.outputs]`). An empty `output` returns `InvalidArgs` (for
every screen it is `FollowDesktopOnLock("")`). Without a choice for every screen the mark has
nothing to override and is dropped; choosing again for every screen drops it too.

## `GetLock() → (s json)`

A `LockReply`: `every`, the choice for every screen or `null`, and `outputs`, the choices of single
screens by screen name. Each choice has `image` (the file) and `item` (its library uid) or
`playlist` (uid), the others `null`, `name`, the title of the item, the file name or the name of
the playlist, and `desktop`, `true` only for a screen that shows its desktop
(`ShowDesktopOnLock`; then the others are `null` and `name` is empty). Changes are announced with the topic `lock`. The Plasma wallpaper plugin, loaded by
the lock screen (`kscreenlocker_greet`), reads it before it asks for a stream, and again on each
`lock` change: with a choice that applies to its screen it asks for `lock:<screen>`, otherwise for
the screen's own stream. `reiwand-ctl lock show [--json]`.

## `HandBackToPlasma() → (u screens)`

Returns `screens`, the number of screens switched. Only in a Plasma session (otherwise `InvalidArgs`:
removing the wallpaper already shows the desktop's own). Runs `ClearWallpaper` for every output and
asks Plasma, through `org.kde.PlasmaShell.evaluateScript`, to change the wallpaper plugin of every
desktop that uses `io.github.reiwand` to `org.kde.image`, which shows the picture Plasma had
configured for that type. A new wallpaper or `RestoreWallpaper` switches the desktops that use
`org.kde.image` back to the Reiwand plugin; this is remembered in a small file next to the
library, so it survives a daemon restart. It cannot choose screens one by one: the scripting
interface names desktops by number, not by output name. `Failed` when Plasma does not answer (for
example from a sandbox that cannot reach it).

## `SetBackground(s color, s output)`

Sets the color shown around and behind the wallpaper of `output`, or of every output when
`output` is empty (this also clears per-output colors). `color` is `#rrggbb`; anything else
returns `InvalidArgs`.

## `SetFilter(s filter, s output)`

Colors the wallpaper of `output` with a filter, or of every output when `output` is empty (this
also clears per-output filters); an empty `filter` clears it. `filter` is a JSON object with any of
`look` (`none`, `warm`, `cool`, `golden`, `vivid`, `punch`, `muted`, `matte`, `fade`, `mono`, `noir`,
`sepia`, `dusk`, `mint`, `rose` or `night`, default `none`), `strength` (how much of the look shows,
0 to 1, default 1), `exposure` (stops, −3 to 3), `contrast`, `saturation` and `warmth` (−1 to 1) and
`hue` (degrees, −180 to 180), all 0 by default; unknown keys and values out of range return
`InvalidArgs`. The filter applies to pictures, videos, scenes and pages alike, never changes the
files, is kept per output under `filter` in `config.toml` (a screen with a filter of its own,
`none` included, keeps it when every output changes), like the fit and the color of the screen. It
is folded into the pass that draws the wallpaper on the screen, so it costs no extra
pass, and no filter costs nothing. The phone uses the filter of the `[default]` section.

## `GetSceneProperties(s output) → (s json)`

A `SceneProperties` document for the package shown on `output`, or for `output = ""` the one set in
the `[default]` section: a scene package or a video package (`kind`), its settings as declared in
its manifest (for a video package, the playback settings described in `package.md`), their current
values, its presets and the presets the user saved for it. `NotFound` when no such package is shown
there. `fields` lists the settings in the order to show them (by `order`, each group followed by its
members), with labels, help and option labels in the language of the daemon's locale, `visible`
false while a `visible_if` condition of the setting or of its group does not hold and `enabled`
false while an `enabled_if` condition of the setting or of its group does not hold; `preset_labels`
names the presets of the package the same way and `user_presets` the saved ones (`value` is their
uid). Clients draw their forms from `fields` alone: `decimals` (always given for sliders, 0 for
whole numbers), `step`, `editable` and `log` describe sliders, a `divider` without a label has
`label` `""`, `icon` is one of the icons of `scene.md` or `""`, `group` names the group of a member,
`collapsed` is the initial state of a group, and `accept` the kinds of file a `file` or `folder`
takes. The value of a `file` or `folder` field is the chosen path or `null` (the package's own
asset). A `secret` text never shows its value: `filled` tells whether one is stored. `shared` is
true when the screen follows the values of the `[default]` section rather than its own. `warnings`
lists stored values that the package no longer accepts (a setting removed, retyped or with a
narrower range after an update), which are ignored for their defaults (`scene.md`).

## `SetSceneProperty(s output, s id, s value_json)`

Sets one setting of the package shown on `output` (`""` = the `[default]` section, used by every
output without its own values) to `value_json`, a JSON value of the setting's type: a `#rrggbb` (or
`#rrggbbaa` with `alpha`) string for `color`, a number for `slider`, `true` or `false` for `bool`,
the value of one of the options for `combo`, a string for `text`, and an absolute path or `null` for
`file` and `folder`. Numbers out of range are clamped, aligned to the step and rounded to the
decimals; anything else that does not fit, an unknown `id`, a `button`, `group`, `label` or
`divider`, a file of a kind the setting does not accept or that cannot be read as an image or a
video, and a folder that is not one return `InvalidArgs`. Values are stored per package id in the
configuration file (see "Configuration file"), never inside the package: in the section of `output`
when that screen has its own values for the package, otherwise in the `[default]` section that the
screens without their own values share. They apply at once without reloading the scene. A `secret`
text is stored in the Secret Service instead, once for every output (`KeyringUnavailable` without
one), and `""` forgets it; it never reaches the configuration, the logs or exported settings.
