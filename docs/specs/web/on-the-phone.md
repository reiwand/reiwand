# On the phone

The Android app shows the same packages with the system WebView, the engine Android keeps up to
date for every app, instead of `reiwand-web`: packages never bring an engine, and the app starts no
process of its own. The package, `window.reiwand`, the privacy settings and the notice are the same;
these are the differences:

- **Drawing.** The page is laid out at the size of the wallpaper on a private virtual display whose
  output is the wallpaper's own surface, so its frames reach the screen without copies. The change
  to or from a page has no fade.
- **Addresses.** The page lives at `https://<16 hex>.wpak.invalid/`, a host made from the package id
  that never resolves, and every request goes through the app: it serves only the files the package
  lists (never `manifest.json`), the interface at `/:reiwand/api.js` (added to every HTML file, after
  its doctype) and the files the user chose as `https://<host>/:user/<property>/<token>` instead of
  `wpak-user://`. Both are opaque to the page. The content security policy is the one above with
  `'self'` in place of the schemes of the desktop.
- **Network.** The app's request filter only sees what the WebView loads: WebSocket and WebRTC do
  not pass through it. A page with network granted reaches only its declared sites because every
  WebView of the app goes through a proxy override that leads nowhere but for them; on a WebView
  without proxy override (`PROXY_OVERRIDE`) every page runs without network. The bridge to the app
  (`reiwandHost`) is only given to the frames of the package's own origin (`WEB_MESSAGE_LISTENER`;
  a WebView without it gives it to every frame).
- **Frame rate.** `requestAnimationFrame` is paced at the cap by the injected script, installed before the page's own scripts and in every frame (an inner frame hands its frames to its parent's capped function; workers keep their own, unpaced, `requestAnimationFrame` of an `OffscreenCanvas` only when `hide_gpu` is off); CSS animations
  and videos run at the rate of the display.
- **Lifecycle.** A page is visible while its screen is shown and the phone neither saves power, nor
  runs low on battery, nor is very hot; otherwise the WebView is paused (`visibilityState` is
  `"hidden"` and `onPause(true)` fires). After 5 minutes hidden it is closed and loads again when
  shown. A renderer that crashes loads the page again, up to three times. Touches reach the page
  only after its first paint.
- **Media.** `onMedia` gets the same as on the desktop from the media sessions of the phone, once
  the user allows `media` (and `media_network` for a page with network origins) and gives the app
  notification access; the cover arrives as a PNG the page turns into a `blob:` address. The app
  starts reading the players again by itself when notification access comes back, and tries again
  six times (1 s, then each wait doubled) when the system refuses to list them; taking access away
  stops the reading at once and the settings screen follows both changes.
- **Sandbox.** Layers 1 to 4 are the same, enforced by intercepting every request. Network loads
  are blocked in the WebView unless the package declares origins and the user allowed `network`,
  and only those origins pass; cookies, service workers, safe browsing and the WebView's metrics
  are off, and peer connections (WebRTC) never work, with or without the network: every response
  carries `webrtc 'block'` in its content security policy, which every frame inherits, including
  `about:blank` and `srcdoc` ones, on engines that honour it (WebView 113 does not: it reports the
  directive as unknown and ignores it), and the guard below removes them from the page, its frames,
  the frames inside shadow trees, `window.open` windows and re-opened documents, on every WebView
  the app can run on (Android 12 and later, whose WebView has `addDocumentStartJavaScript`; the
  guard also runs from the page's interface script where it does not). Nothing in the privacy
  notice depends on the engine's version. The renderer runs in the WebView's isolated process.
- **Privacy.** `hide_zone` and `hide_language` are applied by a guard script that runs before the
  page's: `Date`, `Temporal`, `Intl`, the `toLocale…` methods, `localeCompare` and
  `navigator.language` see UTC and `en-US`, also through `constructor`, when an option or the list
  of languages is left empty (`timeZone: undefined`, `[]`) or names no language the engine knows,
  in dates written without an offset (`new Date("2020-01-01T00:00")`, `Date.parse`, read as UTC),
  in `getYear` and `setYear` and in `document.lastModified`. The WebView runs the guard
  at the start of every frame of the page's origin (`addDocumentStartJavaScript`), and the guard
  also applies itself to the frames the page creates, reads through `contentWindow` or reaches
  through `window[n]`, to its workers (each starts from `/:reiwand/worker.js`, which runs the guard
  and then the page's script) and to its worklets. `hide_gpu` uses the same mask as the desktop,
  where the phone's WebView would report the model of the graphics chip. The disguise is by script
  only: what the engine draws itself (date and time pickers, the fonts it picks for a language) and
  the `Accept-Language` of requests keep following the phone. Kept storage is the
  WebView's storage of the page's origin; *Clear data* and removing the last item of the package
  move the page to a new origin and delete the old one, and a page that keeps nothing gets a new
  origin each time it loads, deleted when it closes. The notice is shown on the page of the
  package, above *Set as wallpaper*, in its *Privacy* section with one switch per item and *Clear
  data*, and as a message after importing it.
- **Inputs.** *React to touch* is the pointer (`pointermove` from the finger, as a mouse hover) and
  *React to taps* the clicks: a tap that does not move becomes one real click. With both the page
  gets the touches as they are. The lock screen gives none.
- **Sound.** The page's own sound follows the phone's sound setting for videos: `reiwand.volume` is 0,
  and the page muted, unless video sound is on; then it is the page's *Volume* setting (from the
  author's `playback.volume`, or 0 with `muted`, unless the page has a `volume` property of its own)
  times the fade of `playback.duck`. For that fade a page counts as the wallpaper's own sound only
  while one of its `audio` or `video` elements plays unmuted with a volume above 0 or one of its
  `AudioContext` runs: the page tells the app when that changes, so a silent page no longer hides
  another app playing; a player of the wallpaper that stops still counts as its own for 1 s while
  its stream goes away. `playback.max_fps` caps `settings.fps` as on the desktop.
  `reiwand.openShortcut(prop)` opens the app or `https://` address the user chose, within 2 s of a tap
  on the home screen and with the taps grant, at most once a second. `onAudio` reads the Visualizer of Android, as scenes
  do, at most 30 times a second.
- **Console.** `console` output and errors reach logcat (tag `reiwand`, `page:`), at most 200 lines a
  load.
