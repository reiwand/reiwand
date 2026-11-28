# The page and the program

On this page: `window.reiwand`, testing and debugging.

## The `window.reiwand` interface

It is ready before your scripts:

```js
reiwand.onProperties((changed) => {
  if (changed.color) document.body.style.color = changed.color.hex;
});
reiwand.onPause((paused) => paused ? stop() : resume());
reiwand.onSettings(({ fps, locale, volume }) => adjust(fps, locale, volume));
```

| Member | What it gives |
|--------|---------------|
| `properties`, `onProperties(f)` | All the properties when you register `f`, and afterwards only those that change. A color arrives as `{ hex, r, g, b, a }` (from 0 to 1), a combo as `{ value, label }`, a file as a `wpak-user://` address and a folder as `wpak-user://<id>/`. |
| `fps`, `quality`, `locale`, `volume`, `onSettings(f)` | The frame rate the page is drawn at, the quality level (0 to 3), the language chosen among those of the package, always as a valid language tag (`de-AT`, `en-US`) that can be passed to `Intl`, and the wallpaper volume (0 to 1) for the sound of the page. `f` receives `{ fps, quality, locale, volume }` when registered and when something changes. |
| `paused`, `onPause(f)` | `true` while the page is hidden or paused. |
| `onAudio(f)` | 64 bands per channel, `{ left, right }`, from bass to treble, about 30 times per second. It needs `audio` in `inputs` and the user allowing it; without it, silence arrives. |
| pointer and clicks | Normal browser events: `pointermove` if you declare `pointer`; `click`, only from real clicks and on the screens where the user turns them on, if you declare `clicks`. The keyboard never arrives. |
| `files(property)`, `randomFile(property)`, `onFiles(property, f)` | The addresses of the files of a chosen folder (below). |
| `onAction(f)` | The id of a `button` property that was pressed. |
| `openShortcut(property)` | Opens what the user chose in a `shortcut` property. It only works inside a real click and at most once per second. |
| `onMedia(f)` | What plays in the player (`{ enabled, state, title, artist, album, cover, colors, position, duration, rate }`), with the cover as a `blob:` address and its colors. It needs `media` in `inputs`. Without the permission, `{ enabled: false }`. |
| `rgb.setImage(bytes, width, height)` | Sends an RGBA image to the computer's RGB lighting (it is reduced to 64 × 16, up to 30 times per second). It needs `permissions.rgb` and the user allowing it; it returns `true` only when the colors go out. |

- `requestAnimationFrame` already runs at the rate the user chooses (and slower on battery), in
  the page and in its frames: there is no need to limit it. In a worker it is not limited (only
  what it shows is), so drawing from a worker's `OffscreenCanvas` costs more battery than drawing
  from the page. The frame rate never goes over `playback.max_fps`. A page that does not
  change spends nothing: it draws only when needed.
- With the page hidden, `document.visibilityState` is `"hidden"`, `onPause(true)` notifies and
  frames stop; after 5 minutes hidden it is closed and loaded again when shown.
- `localStorage` and IndexedDB keep data between starts if you declare `"web": { "storage_kib": 64
  }` (from 0 to 5120) and the user does not hide it; without that, nothing survives. What is
  stored stays on the computer, is never synced and is deleted with the package or with *Clear
  data*. If it goes over twice `storage_kib` plus 1 MiB, the page starts without saving anything
  and its error says so.
- `reiwand.volume` is the wallpaper volume for the sound of your page: it can sound without anyone
  touching anything, `<audio>`, `<video>` and `AudioContext` sound at that volume (times the one
  you set) and at 0 it is muted. The audios and videos of the package or of the chosen folder play
  from the engine's memory, which keeps up to 64 MiB in total; a larger file is not kept and is
  read again from the package on every request, so a large video costs one read per seek.
- `web.preload` is a `.js` or `.mjs` of the package that runs before the scripts of the page, and
  `web.background` (`#rrggbb`) is the color seen while it loads.

## Test and debug

`console.log` and the errors of the page come out in three places:

- in the editor, in the *Console* panel, while the preview is on (it keeps
  the last 50 lines);
- in the terminal: `reiwand-ctl scene preview my-wallpaper -o DP-1 --console` prints them as they
  come out, on your screens or on test ones, while something is looking at that output and until
  you interrupt it;
- in the service log, at `debug` level (`RUST_LOG=reiwandd=debug`), with a 50-line bucket that
  refills at 10 per second.

`reiwand-ctl scene preview my-wallpaper --values settings.json` tries property values without
saving them and `--media track.json` invents a player track. The output of `reiwand-ctl state` says
whether the engine's protection is complete (`sandbox`: `on`) or reduced (`reduced`: some container
does not allow the namespaces it uses).
