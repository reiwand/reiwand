# `io.github.reiwand.Playlists1`

Playlist arguments take a full uid, a name (names compare without case) or the last six or more
characters of a uid, as `reiwand-ctl playlist ls` shows it (an ending that fits more than one playlist
is `InvalidArgs`). Rotation runs in the
daemon and needs no client:

- A playlist switches at every `interval` and at every `at` time (local time), on the listed
  `days` only. A time skipped by a daylight saving jump fires right after the gap; a time that
  occurs twice fires once. `Next` and `Previous` work on any day and restart the interval.
- The daemon sleeps until the nearest switch, waking at least every 15 minutes to follow the wall
  clock, and recomputes after a resume from suspend and when the configuration, the power
  source, the pause state, the outputs or the library change. With no switch pending (no
  playlist shown, paused outputs, empty playlists, no timer) it does not wake at all.
- When the wall clock is set or the time zone changes (`org.freedesktop.timedate1` reports a new
  `Timezone`; in the Flatpak, whose `/etc/localtime` keeps the zone the sandbox started with, the
  daemon follows `/run/host/monitor/timezone` instead), the next `at` time is computed again at
  once in the new zone, from the zone name the signal carries; an interval keeps counting from
  the last switch (or start) unless the clock moved back past it, so a playlist with both an
  interval and `at` times never switches at an `at` time of the old zone, and an overdue switch
  happens once.
- While a `scan`, `import`, `analyze` or `provider_sync` job runs, library changes resolve the
  playlists shown at most once every 3 seconds.
- Each output has its own position (`synced = false`), or every output showing the playlist
  shares one (`synced = true`) and shows the same item. Positions are kept in the library, so a
  restart shows the same item (`resume = true`) or the next one (`resume = false`). A position
  that cannot be saved (a full disk, a locked library) is kept in memory and saved with the next
  change, so later library changes never switch the playlist again. `Start` moves a playlist
  with `resume = false` exactly one item, and `SetRule` waits for a switch in progress.
- Adding, removing or reordering items, or a scan, never changes the item shown; if that item
  is gone, the item at its position is shown. Shuffled playlists never repeat an item within a
  cycle.
- While an output is not active (paused, covered, locked or paused on battery) its playlist does
  not switch unless `advance_while_paused` is set; when it becomes active and a switch is
  overdue, it switches once. Disconnected outputs do not switch either, and switch once when
  they come back after a missed switch.
- Missing items, items that cannot be decoded, revoked packages and photosensitive packages while
  `playback.allow_flashing` is off are left out, and so are videos on battery with
  `skip_videos_on_battery`. If a playlist has nothing to show, the output
  keeps its last picture, or shows its background color with the reason in
  `GetState().outputs[x].error`.
- A scene that plays without one of its package effects, because the graphics driver refused it or
  the GPU stopped while drawing it, keeps `playing`, leaves `error` empty and lists every effect
  left out, with the reason, in `GetState().outputs[x].left_out`; a package material shader that
  cannot be built is listed there too, and its models are shaded with `pbr`.
- `GetState().outputs[x]` reports the `item` shown and `playlist` (`uid`, `name`, `position` in
  the cycle, `count` of playable items, `next_at`). Every shown item is recorded in `History`.

## `List() → (s json)`

A `PlaylistsReply`.

## `Get(s playlist, u offset, u limit) → (s json)`

A `PlaylistPage`: the playlist and a page of its items. Manual playlists list their entries in
stored order (missing items included); smart playlists list the current result of their query.
The item at `items[i]` is entry `offset + i`. `limit` is `0..=500`.

## `Create(s name, s query) → (s uid)`

Creates a manual playlist (`query = ""`) or a smart one. The query is validated and normalized
as in `Library1.Search`. `Exists` for a name in use.

## `Rename(s playlist, s name)`

Changes the name only.

## `SetQuery(s playlist, s query)`

`""` turns a smart playlist into a manual one; a query turns a manual playlist into a smart one.
The manual entries stay stored and come back when the query is cleared.

## `Delete(s playlist)`

Stops the playlist on its outputs (the picture shown stays), then deletes it with its positions.

## `AddItems(s playlist, as uids)`

Appends items to a manual playlist; an item may appear more than once. `InvalidArgs` for a smart
playlist.

## `AddFolder(s playlist, s path, b follow) → (u count)`

Adds every picture and video inside the folder `path` (an absolute local path) and its
subfolders to a playlist. A folder the library does not cover yet becomes a recursive source
(or a flat one when it contains another source) and is scanned before the call returns, so a
large folder takes a while. With `follow` false the items are appended to a manual playlist
(`InvalidArgs` for a smart one). With `follow` true the playlist becomes the smart playlist
`folder:"<path>" -kind:package`, so files that appear in the folder join it by themselves;
`InvalidArgs` when the playlist already holds a list or another search (follow a folder with a
new playlist). Returns how many items the folder has. A path with a quote in its name is refused.

## `Rotate(s name, as uids, s folder, b follow, s every, s output) → (s uid)`

The shortest path to "change the wallpaper every N minutes": makes a manual playlist from the
items `uids` and the folder `folder` (either may be empty, not both; `follow` needs a folder and
no items), sets its `interval` rule to `every` (such as `10m`; longer than zero, at least 10 s)
and starts it on `output` (empty for every output). The playlist is named `name`, the folder's
name when `name` is empty (`Rotation` with no folder either; the app and `reiwand-ctl` send a name in
the user's language instead), and gets a number when the name exists. When any step fails the new
playlist is deleted again. Returns the uid of the playlist.

## `RemoveItems(s playlist, as uids)`

Removes every occurrence of each item from a manual playlist.

## `RemoveEntries(s playlist, au indexes, as expected)`

Removes the entries at `indexes` when `expected[i]` is the uid stored at `indexes[i]`;
otherwise nothing changes and `Conflict` is returned.

## `MoveItem(s playlist, u from, u to, s expected)`

Moves the entry at `from` to `to` when the entry at `from` is `expected`; otherwise `Conflict`.

## `SetRule(s playlist, s key, s value)`

Sets one rotation rule; the keys are listed in `GetSchema().rules`. Changing `synced` keeps the
item shown as the start of the new positions; any change restarts the interval.

| Key | Value | Default |
|-----|-------|---------|
| `order` | `sequential`, `shuffle` | `sequential` |
| `interval` | `0s` (no timer) or `10s` to `30d`, such as `90s`, `1h30m`, `1d` | `30m` |
| `at` | comma-separated `HH:MM` local times, at most 24; `""` clears | `""` |
| `days` | comma-separated weekdays `Mon` to `Sun`; `""` = every day | `""` |
| `synced` | `true`, `false`: one cursor for every output showing the playlist | `false` |
| `resume` | `true`, `false`: show the current item again after a restart | `true` |
| `advance_while_paused` | `true`, `false` | `false` |
| `skip_videos_on_battery` | `true`, `false` | `false` |
| `transition_ms` | `0` to `10000`; `""` = `transition.duration_ms` | `""` |

## `Start(s playlist, s output)`

Assigns the playlist to `output` (`""` = every output, clearing the wallpapers and playlists of
single outputs), writes it as `playlist` of that configuration section and clears its `image`
and `preset`, then shows the current or first item at once (the next one with `resume = false`)
and restarts the interval. `Failed` (`the playlist has no items to show`) when nothing is
playable; nothing changes then. The assignment and the first switch happen together, so a
playlist with `resume = false` moves exactly one item.

## `Stop(s output)`

Removes the playlist of `output` (`""` = every output that has one) and keeps the item shown as
the fixed wallpaper (written as `image`). An output without playlist is left as it is.

A playlist that becomes empty while it is shown (its items removed, or a smart playlist whose
query matches nothing for a while) keeps its last item on screen as long as that file exists, so a
passing scan does not blank the screen; once the file is gone the output reports `the playlist has
no items to show` and shows its background colour.

## `Next(s output)`

Shows the next item on `output` (`""` = every output with a playlist) and restarts the
interval. A synced playlist moves on every output that shows it. `NotFound` for an output
without playlist.

## `Previous(s output)`

Goes one step back on `output`, like `Next`: from the first item of a shuffle cycle to the last
item of the previous cycle; at the very start nothing changes. A shuffled playlist keeps the last
32 items it showed and the last 32 it stepped back over, with their place in the shuffle, so
`Previous` and then `Next` show exactly the items shown before, in order, also after the list
changed or reiwandd restarted; an item that left the list is skipped.
