# The minimum web wallpaper

On this page: the smallest page and where it shows.

## The minimum

```
my-wallpaper/
  manifest.json
  index.html
  app.js
```

The manifest is that of `docs/authoring/examples/web-clock`: it carries all the fields, because
none is omitted (empty ones are written empty or `null`) and it passes `pack check`.

```json
{
  "authors": [],
  "changes": null,
  "content": {
    "descriptors": [],
    "rating": "general"
  },
  "credits": [],
  "default_locale": "en",
  "description": "",
  "entry": "index.html",
  "id": "org.example.web-clock",
  "inputs": [],
  "kind": "web",
  "license": "CC0-1.0",
  "origin": {
    "derived_from": [],
    "kind": "original",
    "provider": null,
    "source_url": null
  },
  "photosensitive": false,
  "platforms": [],
  "presets": [],
  "properties": {
    "color": {
      "alpha": false,
      "default": "#f2f5ff",
      "enabled_if": {},
      "label": "@reiwand.color",
      "order": 0,
      "quality": {},
      "type": "color",
      "visible_if": {}
    }
  },
  "requires": {
    "engine": null,
    "features": [
      "engine_strings",
      "web"
    ],
    "gpu": null,
    "packages": []
  },
  "schema": "reiwand.wpak/3",
  "strings": {},
  "tags": [],
  "title": "Web clock",
  "version": "0.1.0",
  "web": {
    "background": "#101828",
    "preload": null,
    "storage_kib": 0
  }
}
```

`reiwand-ctl scene new my-wallpaper --from web` (or *Web page* on the *Scenes* page of the app)
creates this folder with a sample page (`index.html`, `app.js` and a manifest without `previews`)
that you can edit in the app or with any text editor. It does not create the thumbnail: `pack check`
passes without it, but add it with an image of your own, which `scene thumbnail` copies to
`assets/preview.png`, shrinks if needed and writes into `previews.thumbnail`. The limit is png, jpg
or webp of up to 1024 pixels per side and 1 MiB; with a 1920-pixel image written by hand into the
manifest, `pack check` fails with "previews.thumbnail is not valid". Then:

```
reiwand-ctl scene thumbnail my-wallpaper capture.png
reiwand-ctl pack check my-wallpaper
reiwand-ctl pack build my-wallpaper -o clock.wpak
reiwand-ctl lib import clock.wpak --wait
```

`pack check` explains what is missing, `pack build` computes `requires.features` (here, `web`) and
assembles the package, and `lib import` puts it in the library. Paths are relative: `<script
src="app.js">`, `fetch("data.json")`. The rest of the way (signing, publishing) is that of any
package: `package.md`.

A web wallpaper is a `kind: web` package whose `entry` is an `.html` or `.htm` page. It can carry
images (also `svg`, `hdr` and `exr`), videos, `html`, `js`, `mjs`, `css`, `json`, `wasm`, sounds,
fonts, models (`glb`, `gltf`, `bin`, `obj`, `mtl`, `fbx`), `basis`, shaders (`glsl`, `vert`,
`frag`, `wgsl`), `csv`, `xml` and `txt`. Never programs or libraries; a `.wasm` has to start with
its `\0asm` signature.

## Where it shows

Pages are drawn by a helper program, `reiwand-web`, with Qt WebEngine (Chromium, Qt 6.8 or newer).
Where that engine is not present (Ubuntu 24.04), the daemon rejects web wallpapers with the reason
instead of leaving the screen blank, and says so in `reiwand-ctl lib apply`, in the app and in the
error of the screen. On the phone the system WebView is used.

Codecs: for the video to play on all systems, encode it in VP9, VP8 or AV1 (WebM or MP4 with AV1)
and the sound in Opus or Vorbis. H.264 and AAC only play where the system brings a full `ffmpeg`
(for example Arch) and are expected not to on Fedora (not checked with a real video) nor in the
Flatpak; the phone does play them, but a package plays everywhere only with the free formats. Each
page is tested with `reiwand-ctl` as explained above, on a Fedora or an Arch with Qt WebEngine, not
in the phone's WebView.

A web wallpaper that stops responding restarts by itself:

- the helper program asks the page every 5 s; if it does not answer in 10 s (a script stuck in a
  loop), it stops it and loads it again;
- a page that crashes or runs out of memory is loaded again at 1, 4 and 9 s; on the fourth in a
  row it is turned off, the screen shows the color of `web.background` and its error says why;
- it takes at most one processor; among all its processes it slows down when it goes over 768 MB
  together and is killed when it goes over 1.5 GB; there is no per-process limit.
