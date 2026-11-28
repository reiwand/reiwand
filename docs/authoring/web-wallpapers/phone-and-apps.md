# Phones and existing applications

On this page: web wallpapers on the phone and turning an application into one.

## On the phone

The Android app shows the same package with the system WebView (the Chrome engine that Android
keeps up to date), with the same `window.reiwand` interface. It is worth keeping in mind:

- The page lives at an `https://…wpak.invalid/` address and chosen files arrive as
  `https://…/:user/<property>/…` instead of `wpak-user://`: treat them as opaque addresses and use
  relative paths for your files.
- `requestAnimationFrame` runs at the rate the user chooses, but CSS animations and videos run at
  the screen's: to save battery, animate with `requestAnimationFrame`.
- The finger is the pointer (`pointermove`, like a mouse passing over) if the user turns on *React
  to touch*, and a touch without moving is a `click` if they turn on *React to taps*. Touches
  arrive after the first drawing of the page. The lock screen gives none.
- The sound of the page plays only if the user turns on the sound of videos ("Play the sound of
  videos and scenes"); if not, `reiwand.volume` is 0.
- `RTCPeerConnection` does not exist (it is blocked on all pages) and `console` goes to logcat
  (`adb logcat -s reiwand`, up to 200 lines per load).
- WebGL depends on the phone: check that `getContext("webgl")` does not return `null` and have a
  simpler drawing for that case.
- The WebView is paused with power saving, with low battery or with a lot of heat: `onPause(true)`
  notifies.

## Turning an application into a web wallpaper

Reiwand never runs programs from a package. A wallpaper made as an application (an executable that
draws behind the icons) is converted like this:

1. Export the project to web from its engine: the most used game engines and 3D libraries generate
   HTML with WebGL or WebAssembly (`.wasm`, which must start with its `\0asm` signature).
2. Put the result in the package folder with `index.html` as `entry` and change absolute paths to
   relative ones.
3. Replace what it read from the system with the interface: its options with `properties` and
   `onProperties`, its audio analysis with `onAudio`, its mouse with the pointer events.
4. Remove what cannot be done: files of the computer (use a `folder` or `file` property),
   undeclared network, windows, keyboard and processes.
5. Check that it draws little: limit the work per frame, pause on `onPause` and do not make fast
   `setInterval` loops, which show on the battery.

Executables and libraries (exe, dll, so, sh, bat, AppImage…) are rejected when the package is
assembled.
