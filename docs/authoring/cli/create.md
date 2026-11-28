# Create, change and import

On this page: making a scene and changing it with commands.

## Create

### `scene new`: a new scene folder

```
reiwand-ctl scene new my-scene --from clock --title "My clock" --locale de
```

It prints nothing and leaves `manifest.json`, `scene.json` and `assets/` in `my-scene/`.

| Option | What it does |
|--------|--------------|
| `--from` | The starting point: `empty` (the default), `particles`, `parallax`, `clock`, `web` (a web page) or `preset` (the values of another package). |
| `--id` | The package id. Without it, `org.example.<folder name>`. |
| `--title` | The title. Without it, the folder name. |
| `--locale` | The language of the title (`de`, `pt-BR`). Without it, the desktop's. |

The folder it produces already passes the check (`pack check`). Change the `id` before publishing.

### `scene open`: edit a copy of a package

```
reiwand-ctl scene open my-copy --from clock.wpak
```

```
Clock with script (org.example.clock.copy 1.0.0): 3 layers, 0 problems
```

It unpacks the `.wpak` into a new folder as an unsigned copy, with the id `<id>.copy`. Without
`--from`, it describes a folder that already exists. More options: `--id` (another id for the
copy), `--facts` (prints in JSON the license, the verdict `allowed` or `refused` and the id of the
copy, without writing anything), `--mine` (confirms that the package is yours or that its author
allows the copy, for licenses that do not allow derivative works) and `--json`. See “Edit a copy” in
`package.md`.

## Changing the scene with commands

These are the commands the tutorial in [README.md](../README.md) uses to build a scene without the
editor. They change the folder immediately and do not need the daemon.

### `scene act DIR add`: a new layer

```
reiwand-ctl scene act my-scene add_layer '{"kind":"image","name":"Background","file":"sky.png"}'
```

```
background
```

It adds the layer and writes its id, the name the other commands use for it. The id comes from
`--name` (or, if you do not give it, from the layer type) in lowercase. Layers, assets, particle
systems and timelines share their ids: if any of them already has the one that would result, `_2`,
`_3`… is appended and the command prints the one it used. An image you import with `--file` is
named after its file, so `--file sunset.png --name Sunset` gives the layer `sunset_2`: always look
at what the command prints. With `--id` you pick the id yourself; if it is already in use, the
command fails with `the id background is in use` and changes nothing.

An image layer without a size is shown at its real size and centered, not at the canvas size. To
cover the canvas: `reiwand-ctl scene act DIR layer`. A new particle layer already comes with
falling snow over the whole canvas (see [particles-and-sound.md](../particles-and-sound.md)).

### `scene show`: read any value

```
reiwand-ctl scene show my-scene /scene/layers/0
reiwand-ctl scene show my-scene /manifest/title
reiwand-ctl scene show my-scene
```

It prints in JSON the value at a JSON Pointer; without a pointer, the two documents together. The
pointer **always starts with `/scene`** (the content of `scene.json`: `/scene/layers`,
`/scene/timelines`, `/scene/particles/snow`) or with `/manifest` (that of `manifest.json`:
`/manifest/title`, `/manifest/inputs`). `/layers` does not exist and the command says `Error:
nothing is at that pointer`; the same happens with a list that has nothing yet, such as
`/scene/timelines` in a scene with no timelines. Layers are counted from 0 starting at the bottom:
the first one you added is `/scene/layers/0`. `scene act DIR set` writes a value in any of those
places, and `pack check` says what is wrong with the same kind of pointer.

## Import

### `scene import`: a file into the scene

```
reiwand-ctl scene import my-scene background.png --id sky
```

```
sky
```

It copies an image, a video, a sound, a font or a `.cube` table into `assets/` and declares it as
an asset of the scene. It prints the asset id (the file name if you do not give `--id`). `--linear`
reads the image as data (a normal map, a mask) and not as colors; `--atlas ROWS,COLUMNS` splits it
into a sprite sheet.

### `scene act DIR import_script`: a script into the scene

```
reiwand-ctl scene act my-scene import_script '{"file":"my-scripts/follow.js"}'
reiwand-ctl scene act my-scene import_script '{"template":"timer"}'
```

```
scripts/follow.js was added
scripts/follow.js now uses pointer
```

It copies the module into `scripts/` and writes in `scene.json` which events it handles and which
parts of `reiwand` it uses. With `--template` it starts from a template: `empty`, `media`, `touch`,
`timer`, `audio`, `spawn`, `wave`, `daytime`, `setting` or `launches`. See `scripts.md`.

### `lib import`: a package into the library (with the daemon)

```
reiwand-ctl lib import clock.wpak --wait
```

```
job 1 import done: 0 failed, 1 imported
```

It installs the package (or an image or a video) in your library. Without `--wait`, it prints the
job number and returns right away.
