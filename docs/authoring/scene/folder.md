# The folder, the clock and the assets

On this page: what a scene folder holds, the clock of the scene and the files it uses.

## What is in the folder

A scene is a folder with three things:

- `manifest.json`: the name, the author, the license and the user properties. It is explained in
  [package.md](../package.md).
- `scene.json`: the layers and everything that moves. This is what this page describes.
- `assets/`: the images, sounds, fonts and models that the scene uses.

`scene.json` is a JSON object and carries all its fields, also those you do not use (an empty
list, an empty object or `null`): the loader rejects the file and names the missing field. The
editor and `reiwand-ctl scene new` write them for you.

| Field | What it is |
|-------|------------|
| `schema` | Always `"reiwand.scene/2"`. |
| `canvas` | The canvas: size, fit, background and effects of the whole scene. |
| `clock` | The clock: how many frames per second and which moment serves as the poster. |
| `assets` | The assets, each with an id. |
| `layers` | The layers, from bottom to top. See [layers.md](../layers.md). |
| `timelines` | The timelines that move values. |
| `particles` | The particle systems. See [particles-and-sound.md](../particles-and-sound.md). |
| `effects` | Effects of the package itself. See [effects.md](../effects.md). |
| `camera3d` | The camera of the 3D models. See [3d.md](../3d.md). |
| `audio` | How the scene reacts to sound, below. |
| `scripts`, `templates`, `pools`, `ext` | Code and templates. See [scripts.md](../scripts.md). |

If you write a field that does not exist, a value out of range or a broken reference, the scene is
rejected with the path of the field, for example `/layers/2/effects/0/params/radius is 300,
expected 0..=128`. Check your scene with:

```
reiwand-ctl pack check MY-FOLDER
```

It prints nothing if all is well, and it also compiles the scripts of the scene without running
them (a syntax error comes out with the file and the line). To read or change any field from the
terminal, the pointer starts with `/scene` (what is in `scene.json`: `/scene/layers/0`,
`/scene/canvas/size`) or with `/manifest` (the manifest): `reiwand-ctl scene show MY-FOLDER /scene/canvas`.

Colors are written `"#rrggbb"` or `"#rrggbbaa"` (always with 6 or 8 digits: `"#fff"` is
rejected).

Positions are canvas pixels with the origin at the top left. Angles are degrees and turn
clockwise.

## The clock

```json
  "clock": { "max_fps": 30, "poster_time": 2, "seed": 7, "step": 60 }
```

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `max_fps` | 60 | 1 to 240, or `"display"` (also 0) | Cap on frames per second. `"display"` follows the rate of each screen. |
| `poster_time` | 0 | 0 to 86400 s | The instant shown where motion is not wanted and as the library thumbnail. |
| `seed` | 0 | integer, or `null` | The seed of whatever is done at random. With `null` it changes on every start. |
| `step` | every minute | 1 to 3600 s | How often the values that follow the day change (see "Values"). |

A scene of a clock or an almost still image does not need 60 frames: lower `max_fps` (`1` to
`30`) and save battery. What the user sets (`playback.max_fps`) overrides your value.

## The assets

The assets are the files the scene uses. Each has an id of up to 32 lowercase letters, digits or
`_`, and a path inside the folder.

```json
  "assets": {
    "sky": { "path": "assets/sky.png", "filter": "linear", "wrap": "clamp", "linear": false,
               "mipmaps": null, "face": null, "rects": [], "frames": null, "fps": null, "durations": [],
               "animated": false, "shapes": {} }
  }
```

Layers name the asset by its id (`"image": "sky"`). A scene has at most 256 assets.

| Type | Formats | Limits |
|------|---------|--------|
| Image | PNG, JPEG, WebP, GIF, BMP, KTX2 | The images of a scene take at most 512 MiB once opened. |
| Font | TTF, OTF, TTC, OTC, WOFF2 | Up to 32 MiB each. |
| Video | WebM, MP4, MKV | Stored as is, without recompressing. |
| Sound | OGG (Vorbis or Opus), OPUS, FLAC, WAV, MP3 | See [particles-and-sound.md](../particles-and-sound.md). |
| Color table | `.cube` | `LUT_3D_SIZE` from 2 to 65, up to 16 MiB each and 64 MiB in total. |
| 3D model | `.glb` (binary glTF) | Up to 64 MiB each. See [3d.md](../3d.md). |

`scene.json` weighs at most 8 MiB.

Fields of an asset:

| Field | What it does |
|-------|--------------|
| `path` | The path of the file. Required. |
| `linear` | `true` for images that are data and not colors: depth maps, normal maps, flow maps, masks. |
| `filter` | `linear` (smooth) or `nearest` (sharp pixels). |
| `wrap` | What happens outside the image: `clamp`, `repeat` or `mirror`. With `repeat` or `mirror`, an image layer larger than the image is filled with copies instead of stretching it. |
| `mipmaps` | `true` or `false` forces the small copies for images that are seen smaller. |
| `atlas`, `rects` | A sprite sheet: `[rows, columns]` of equal cells, or up to 256 rectangles `[x, y, width, height]`. Not both at once. |
| `frames` | How many cells are played, from 1 to 256. |
| `fps`, `durations` | Frames per second (0.1 to 120) or a duration per frame (0.001 to 60 s). Not both. |
| `animated` | `true` plays an animated GIF or WebP with its timings; up to 256 frames. |
| `face` | Which font of a `.ttc` or `.otc` collection (0 to 255). |
| `media` | `current` or `previous`: the cover of what is playing or played before. `path` is the image while nothing is playing. |
| `shapes` | In a `.glb`, blend shapes sculpted separately. See [3d.md](../3d.md). |

Without `fps` or `durations`, a sheet shows its first frame. An image layer that shows a sheet
plays its frames with the scene clock and controls them with `playback` (see
[layers.md](../layers.md)).

Each image is opened once and uploaded once to the graphics card, even if the scene is seen on
several screens. A video has a single decoder shared by all the layers that show it.

System fonts are named by family; only `sans-serif`, `serif` and `monospace` surely exist
everywhere.

### KTX2 textures

An image asset can be a `.ktx2` file with a Basis Universal texture: an image, a list of up to 256
frames (arranged in a grid, like those of an animated GIF) or a six-face cube, which becomes a
panorama twice as wide as it is tall and is valid wherever a panorama is, such as the `env` of a
model (see [3d.md](../3d.md)). It measures up to 16384 pixels per side, has up to 15 levels and
weighs up to 256 MiB. The program converts it to the format the graphics card reads best, and a
scene of KTX2 textures takes about a quarter of the video memory of the same images as PNG.
`linear`, `filter`, `wrap` and `atlas` apply as with other images, and with `animated: true` the
frames play with the timings of the file. An HDR texture keeps its light above white in canvases
with `hdr`.

They are made on the desktop, from the editor's KTX2 conversion or with:

```
reiwand-ctl scene asset MY-FOLDER sky --ktx2 --codec uastc --quality 80
reiwand-ctl scene asset MY-FOLDER panorama --cube
```

`--dry-run` only reports the sizes and what each class of card receives, and `--faces` makes the
cube from six square images. The phone only reads them, it does not create them: a package made on
the desktop looks the same on the phone.
