# `io.github.reiwand.Rgb1`

Colors for keyboards, mice and other RGB lighting go through an OpenRGB server (its SDK protocol,
versions 0 to 4, with a client of the daemon's own). Nothing is sent and no connection is made while
`rgb.enabled` is `false` (the default); `rgb.server` is `127.0.0.1:6742` by default, and an address
on another computer sends the colors over the network, which the app says next to the field. The
daemon connects when a granted package sends colors, when a covered wallpaper that holds the grant
and sends colors would follow with `rgb.when_covered` set to `follow` (never for an image, a video, a
package without colors or an output paused, locked or on battery), on `Connect` or `Test`, names itself
`reiwand`, reads the mode each device is in (and its colors) and tries again 10 s after a
failure. A device goes to its direct mode only when colors are first painted on it, and gets its own
mode back (the colors too when that mode sets each light) as soon as nothing sends colors any more:
when the package that sent them stops or changes to one without colors, its output goes away, RGB
lighting is turned off, the server changes, after a test with nothing else to paint, and when the
daemon stops. With `rgb.when_covered` set to `follow`, the connection closes again, giving the devices
their own mode back, as soon as nothing wants the lights: no covered wallpaper wants them and no output
that is shown and running sent colors since it last stopped (the session locks or pauses, the computer
goes on battery, the wallpaper changes to one without colors or the output goes away), whether a covered
or a shown wallpaper opened it; while one shown output keeps sending colors it stays open, whichever
output painted last. The daemon then sleeps until something asks for the lights again. A strip of
at most 64 × 16 pixels comes from the shown scene or page of each output that holds the `rgb` grant
(`Inputs1`), at most 30 times a second for all of them together; the daemon spreads it over each
device: a zone with a matrix takes the cell of the strip under each key, the other zones the column
under each light, averaged over the height. `rgb.place.<device>` gives a device its own part of the
strip and `rgb.place.<device>#<zone>` one of its zones (counted from 0 in the order of `devices`):
the matrix or the line of lights is laid over that rectangle, and the line runs `right`, `left`,
`down` (each light takes a row, averaged over the width) or `up`; `off` leaves a device out (it is
never put in direct mode and keeps its own) or keeps a zone dark. A change of places gives the devices
their own mode back and paints again at once, without a new connection.
While a window covers the output that sends the colors and the wallpaper pauses for it (`pause.when_covered`), `rgb.when_covered` decides: `pause` (the
default) sends nothing until it shows again and lets the connection rest without even listening for
device changes, leaving the lights as they are (gentler than switching the devices back to their own
mode and again to direct mode); `follow` keeps them following the scene without drawing it.
Previews (`preview:*` outputs) keep their strip for the editor but never send it.

### `GetStatus() → (s json)`

`{enabled, server, local, connected, error, devices, source, frames, testing, strips, when_covered,
held, places, waiting}`: `local` is
`true` for addresses on this computer, `error` the last failure (`null` when fine), `devices` lists
`{index, name, key, kind, leds, zones: [{name, leds, matrix: [w, h] | null}], own_mode, taken}` as the
server names them, with `key` the name that `rgb.place.*` uses: the device name, or, when several
devices share a name, that name followed by its serial in parentheses (its order among them, from
1, when the serials are missing or equal), `own_mode` the name of the mode the device had before the daemon took it
(`null` when the server names none; kept in `~/.local/share/reiwand/rgb-modes.json` (mode 600) while the device is taken, so a daemon that was killed, or a server that did not answer, gives it back at the next connection while nothing sends colors) and `taken` `true` while it is in direct mode for the lights,
`source` the output whose colors were sent last, `frames` how many were sent and `strips` the
outputs with a strip kept; `when_covered` is `rgb.when_covered` and `held` is `true` while the
lights pause because the output they follow is covered and paused; `places` maps each placed device
or `device#zone` to its `rgb.place.*` value; `waiting` (empty unless RGB lighting is on and
`rgb.when_covered` is `follow`) maps each output whose lights wait to why: `paused` (the user paused
the wallpaper), `locked` or `battery` (the wallpaper pauses while the session is locked or on battery),
for an output whose package sends colors, `ungranted` for a covered one whose package lacks the `rgb`
grant, and `colorless` for a covered and paused output that shows something without colors (an image,
a video or a package without them).

### `Connect()`

Connects (again) now and reads the device list; `Failed` with the reason when RGB lighting is off
or the server cannot be reached.

### `Test(i device)`

Shows a moving rainbow on one device (`-1` for all of them) for 3 s, then the last colors again;
`Failed` when it is off, cannot connect or the device does not exist.

### `GetStrip(s output) → (u width, u height, ay rgb)`

The last strip of an output (`RGB` bytes, rows from the top), also for previews and packages without
the grant, for the editor; `NotFound` when the output has none.
