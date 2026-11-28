# Package

The package's data sheet (`/manifest`): who made it, under what license, what permissions and inputs it uses, how it looks. For the properties the user chooses, see [User properties](user-properties.md#user-properties).

## `manifest`

Edits the package's data sheet. It is a single action with several variants: you pass an object with the variant's name and inside it its arguments. Each variant rejects what the sheet's checks reject, and says which field fails.

### variant `origin`

Says where the package comes from. It replaces the previous origin.

| Argument | Type | What it means |
|-----------|------|---------------|
| `kind` | text (required) | `original` (you made it from scratch), `derived` (based on others' works), `imported` (brought from elsewhere) or `provider` (from an online provider). |
| `derived_from` | list of objects | With `derived`: works it is based on, each with `author` (required) and, if you want, `title`, `url` and `license`. |
| `provider` | object | With `provider`: `name` (lowercase, digits or `-`), `item_url` (an `https://` address), `credit` (text without `<` or `>`) and, if you want, `terms_url`. |
| `source_url` | text | Address it was brought from. |

```sh
reiwand-ctl scene act my-scene manifest '{"origin":{"kind":"derived","derived_from":[{"author":"Alex Morgan","title":"Aurora","url":"https://example.org/aurora","license":"CC-BY-4.0"}]}}'
reiwand-ctl scene act my-scene manifest '{"origin":{"kind":"imported","source_url":"https://example.org/item"}}'
reiwand-ctl scene act my-scene manifest '{"origin":{"kind":"provider","provider":{"name":"gallery","item_url":"https://example.org/item/1","credit":"Alex Morgan"}}}'
reiwand-ctl scene act my-scene manifest '{"origin":{"kind":"original"}}'
```

### variant `license`

Sets the package's license.

| Argument | Type | What it means |
|-----------|------|---------------|
| `license` | text (required) | An SPDX expression of up to 256 characters, such as `CC-BY-4.0`. |

```sh
reiwand-ctl scene act my-scene manifest '{"license":{"license":"CC-BY-4.0"}}'
```

### variant `tags`

Changes the tags by which the package is found.

| Argument | Type | What it means |
|-----------|------|---------------|
| `tags` | list of texts (required) | The tags. |
| `add` | yes/no | `true` adds them to the existing ones. |
| `remove` | yes/no | `true` removes these from the existing ones. Without `add` or `remove`, it replaces them all. |

```sh
reiwand-ctl scene act my-scene manifest '{"tags":{"tags":["calm","blue"]}}'
reiwand-ctl scene act my-scene manifest '{"tags":{"tags":["night"],"add":true}}'
reiwand-ctl scene act my-scene manifest '{"tags":{"tags":["blue"],"remove":true}}'
```

### variant `authors`

Changes the authors.

| Argument | Type | What it means |
|-----------|------|---------------|
| `authors` | list of texts (required) | Up to 64 names. An empty list removes them. |

```sh
reiwand-ctl scene act my-scene manifest '{"authors":{"authors":["Alex","Sam"]}}'
```

### variant `display`

Changes how the package fills the screen and its accent color. Depending on the package type, some fields or others are valid: in a scene, `span` and `accent`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `values` | object | Fields to set: `fit` (`fill`, `fit`, `stretch`, `center`, `tile` or `span`), `zoom` (1 to 8), `position` (two numbers from -1 to 1), `background` (a `#rrggbb` color), `span` (`all-outputs` makes a single image cover all the screens; `per-output` is for images and videos), `flip` (`none`, `horizontal` or `vertical`) and `accent` (a `#rrggbb` color, or `{"prop": "ID"}` with the id of a color property). The fields `fit`, `background` and `flip` are valid only for images and videos, and `zoom` and `position` only for videos. |
| `clear` | list of texts | Fields that are removed. |

```sh
reiwand-ctl scene act my-scene manifest '{"display":{"values":{"span":"all-outputs","accent":"#ff8800"}}}'
reiwand-ctl scene act my-scene manifest '{"display":{"clear":["span"]}}'
```

### variant `playback`

Changes how the package plays. In a scene or web page, `max_fps` (maximum frames per second), `volume` (0 to 1) and `muted` (yes/no). In a video also `speed`, `start`, `end` and `loop`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `values` | object | Fields to set. |
| `clear` | list of texts | Fields that are removed. |

```sh
reiwand-ctl scene act my-scene manifest '{"playback":{"values":{"max_fps":30,"volume":0.5,"muted":true}}}'
reiwand-ctl scene act my-scene manifest '{"playback":{"clear":["muted"]}}'
```

### variant `web`

Only for web page packages (`reiwand-ctl scene new FOLDER --from web`). It adjusts the page: `preload` (a package file that runs before the page), `background` (the color seen while it loads) and `storage_kib` (how much it can store, in KiB).

| Argument | Type | What it means |
|-----------|------|---------------|
| `values` | object | Fields to set. |
| `clear` | list of texts | Fields that are removed. |

```sh
reiwand-ctl scene new my-scene --from web
reiwand-ctl scene act my-scene manifest '{"web":{"values":{"background":"#202020","storage_kib":64}}}'
reiwand-ctl scene act my-scene manifest '{"web":{"clear":["storage_kib"]}}'
```

### variant `network`

Only for web page packages. It lists the `https` origins the page may connect to; with origins in the list, the `network` input is declared by itself.

| Argument | Type | What it means |
|-----------|------|---------------|
| `origins` | list of texts (required) | Addresses such as `https://example.org`. |
| `add` | yes/no | `true` adds them to the existing ones. |
| `remove` | yes/no | `true` removes these. Without `add` or `remove`, it replaces them all. |

```sh
reiwand-ctl scene new my-scene --from web
reiwand-ctl scene act my-scene manifest '{"network":{"origins":["https://example.org"],"add":true}}'
reiwand-ctl scene act my-scene manifest '{"network":{"origins":["https://example.org"],"remove":true}}'
```

### variant `inputs`

Declares which inputs the package reads, so the system gives it access.

| Argument | Type | What it means |
|-----------|------|---------------|
| `inputs` | list of texts (required) | Inputs: `audio`, `pointer`, `clicks`, `media` or `network`. |
| `add` | yes/no | `true` adds them to the existing ones. |
| `remove` | yes/no | `true` removes these. Without `add` or `remove`, it replaces them all. |

```sh
reiwand-ctl scene act my-scene manifest '{"inputs":{"inputs":["pointer"],"add":true}}'
```

### variant `requires`

Declares other packages that this one needs (for example, an assets one).

| Argument | Type | What it means |
|-----------|------|---------------|
| `packages` | list of objects (required) | Each with `id` and, if you want, `version`. |
| `add` | yes/no | `true` adds them to the existing ones. |
| `remove` | yes/no | `true` removes these. Without `add` or `remove`, it replaces them all. |

```sh
reiwand-ctl scene act my-scene manifest '{"requires":{"packages":[{"id":"org.example.base","version":"1.0.0"}],"add":true}}'
```

### variant `preset`

Only for preset packages (`reiwand-ctl scene new FOLDER --from preset`), which store values for another package's properties.

| Argument | Type | What it means |
|-----------|------|---------------|
| `base` | object | The package the settings apply to: `id` and, if you want, `version`. |
| `values` | object | Property values, by id. |
| `clear` | list of texts | Ids of properties whose value is removed. |
| `files` | object | Files given to file-type properties: id of the property and path of the file. |

```sh
reiwand-ctl scene new my-scene --from preset
reiwand-ctl scene act my-scene manifest '{"preset":{"base":{"id":"org.example.base","version":"1.0.0"},"values":{"glow":2}}}'
```

A preset can also be started with everything it needs in a single command: `--base ID` and `--range VERSIONS` (any, without it), `--values FILE` (a JSON with the values), `--license ID` and `--preview IMAGE`. To know what it still lacks before publishing it, see the [`preset_steps`](queries.md#preset_steps) query.

```sh
reiwand-ctl scene new my-scene --from preset --base org.example.base --range '^1.0' --license CC0-1.0
```

### variant `credit`

Credits package files with their license and author. Those with the same license, author and address are gathered into a single credit. To see what is still uncredited, see the [`credits`](queries.md#credits) query.

| Argument | Type | What it means |
|-----------|------|---------------|
| `paths` | list of texts (required) | Package files (`assets/picture.png`) or folders (ending in `/`). Up to 256. They must exist. |
| `license` | text (required) | SPDX license. |
| `attribution` | text (required) | Who made it. |
| `url` | text | `https://` address of the author or the work. |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene manifest '{"credit":{"paths":["assets/picture.png"],"license":"CC-BY-4.0","attribution":"Alex Morgan","url":"https://example.org/alex"}}'
```

### variant `uncredit`

Removes files from the credits. A credit left with no files disappears.

| Argument | Type | What it means |
|-----------|------|---------------|
| `paths` | list of texts (required) | Files or folders. |

```sh
reiwand-ctl scene act my-scene import '{"file":"picture.png"}'
reiwand-ctl scene act my-scene manifest '{"credit":{"paths":["assets/picture.png"],"license":"CC-BY-4.0","attribution":"Alex Morgan"}}'
reiwand-ctl scene act my-scene manifest '{"uncredit":{"paths":["assets/picture.png"]}}'
```

### variant `text`

Writes a text of the package's text table (`strings`) in a language. The texts in the sheet or in the scene that are `@key` read it. To see the table, see the [`strings`](queries.md#strings) query.

| Argument | Type | What it means |
|-----------|------|---------------|
| `key` | text (required) | Key: up to 64 letters, digits, `_`, `.` or `-`. Those that start with `reiwand.` belong to the engine. |
| `language` | text (required) | Language, such as `en` or `pt-BR`. |
| `text` | text | The text. Empty or missing, it removes it. |

```sh
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"en","text":"Hello"}}'
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"de","text":"Hallo"}}'
```

### variant `locale`

Decides whether a language's texts live in the data sheet or in their own file `locale/LANGUAGE.json`.

| Argument | Type | What it means |
|-----------|------|---------------|
| `language` | text (required) | Language. |
| `file` | yes/no (required) | `true` moves them to their file, `false` returns them to the sheet. |

```sh
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"de","text":"Hallo"}}'
reiwand-ctl scene act my-scene manifest '{"locale":{"language":"de","file":true}}'
```

### variant `add_file`

Only for web page, image or video packages: copies a file to the package folder.

| Argument | Type | What it means |
|-----------|------|---------------|
| `file` | text (required) | File that is copied. |
| `to` | text | Package folder to leave it in. Without it, at the root. |

```sh
reiwand-ctl scene new my-scene --from web
reiwand-ctl scene act my-scene manifest '{"add_file":{"file":"picture.png","to":"img"}}'
```

### variant `media`

Only for image or video packages: copies the file and sets it as the one that is shown. Since `scene new` only creates scenes, web pages and presets, the example first turns the package into an image one.

| Argument | Type | What it means |
|-----------|------|---------------|
| `file` | text (required) | The image or video. It must be of the package's type. |

```sh
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/kind","value":"image"}'
reiwand-ctl scene act my-scene manifest '{"media":{"file":"picture.png"}}'
```

### variant `part`

Only for asset packages: changes the title and thumbnail of a part the package offers (`ext.parts`). If the package does not list its parts yet, the first edit writes the list with all the top-level parts and changes the requested one. In the app it is in the package's data sheet, under "the parts it offers".

| Argument | Type | What it means |
|-----------|------|---------------|
| `kind` | text (required) | `layer`, `particles`, `script` or `effect`. |
| `id` | text (required) | The part's id, or the script's path. It must exist in the package. |
| `title` | text | The title, 1 to 128 characters on one line, without `<` or `>`. Empty, it removes it. |
| `thumbnail` | text | A png, jpg or webp image that is reduced to a jpg of at most 1024 pixels per side and saved in `parts/`. |
| `clear_thumbnail` | boolean | `true` removes the thumbnail. |

```sh
reiwand-ctl scene new origin
reiwand-ctl scene act origin add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl pack new-assets origin parts --part layer:sky --id org.example.parts --title Parts --license CC-BY-4.0 --attribution "Alex Morgan"
reiwand-ctl scene act parts manifest '{"part":{"kind":"layer","id":"sky","title":"Sky","thumbnail":"picture.png"}}'
```

## `translate`

Writes, adds or removes the text of a field in a language. It is for the data sheet's texts (`/manifest/title`, `/manifest/description`, `/manifest/changes`), the labels, help and options of user properties, and the media's fallback texts. It is also for a `text` layer's text (`/scene/layers/0/text`), unless it follows a value such as a clock: then the action says so. A layer's text never ends up without text: the last language cannot be removed. When writing in a language other than the package's, a plain text becomes a multi-language text. In the app, the language field next to each text.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path of the field, for example `/manifest/title`. |
| `language` | text (required) | Language, such as `de` or `pt-BR`. Empty is the base text. |
| `text` | text | The text. Empty or missing, it removes that language (the last one of the title cannot be removed). |
| `add` | yes/no | `true` adds the language starting from the text in the package's language, and does not touch one that already exists. Default `false`. |

```sh
reiwand-ctl scene act my-scene translate '{"pointer":"/manifest/title","language":"de","text":"Meine Szene"}'
reiwand-ctl scene act my-scene translate '{"pointer":"/manifest/title","language":"fr","add":true}'
reiwand-ctl scene act my-scene translate '{"pointer":"/manifest/title","language":"fr"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"text","id":"greeting"}'
reiwand-ctl scene act my-scene translate '{"pointer":"/scene/layers/0/text","language":"de","text":"Guten Morgen"}'
```

## `thumbnail`

Sets the image the package is shown with in the library, or its animated preview. It copies the file to `assets/` and notes it in the data sheet. It prints the path it used. To take a capture of the scene instead of giving a file, see [`shot`](#shot).

| Argument | Type | What it means |
|-----------|------|---------------|
| `file` | text (required) | Image file. If it exceeds 1024 pixels or 1 MiB, it is reduced. |
| `animated` | yes/no | `true` uses it as the animated preview: it must be a gif or webp of up to 640 pixels per side, 15 s and 8 MiB. Default `false`. |

```sh
reiwand-ctl scene act my-scene thumbnail '{"file":"picture.png"}'
reiwand-ctl scene act my-scene shot '{"recording":{"seconds":1,"fps":5}}'
reiwand-ctl scene act my-scene thumbnail '{"file":"my-scene/assets/preview-animated.gif","animated":true}'
```

## `shot`

Draws the scene to save a capture as a thumbnail, record a gif or webp as an animated preview, or measure how much it flashes. `still` and `recording` also work for an image package (only `still`: the frame is the image itself, reduced) and a video package (the frame and the recording come from the video). A web page is not drawn in the editor but in the daemon, which shows it with the usual rules: for it use the command `reiwand-ctl scene thumbnail FOLDER --capture [--animated] [--format webp] [--output SCREEN]`, which shows it for a moment on a screen and saves a frame or a recording, or the app's capture button (frame only). With a web page, this action answers with that path. `flashes` is only for scenes. Three variants.

### variant `still`

Saves a frame as a thumbnail (of up to 1024 pixels). In a preset package it draws the base package with the preset's values: a scene with its settings, an image with `zoom`, `x` and `y`, a video with `start` as well; a web page's base is captured with the command above, with `--base FILE`. In a video, `time` is the video's second; without it a frame near the start is taken.

| Argument | Type | What it means |
|-----------|------|---------------|
| `time` | number | Second of the scene. Without it, the scene's still frame (`poster`, see `clock` in [`canvas`](../actions-layers/canvas.md#canvas)). |
| `base` | text | Only in a preset: the installed file of the base package, which is drawn with the preset's values. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid"}'
reiwand-ctl scene act my-scene shot '{"still":{"time":1}}'
```

### variant `recording`

Records a gif or webp of up to 640 pixels and 8 MiB (it is reduced until it fits) as an animated preview.

| Argument | Type | What it means |
|-----------|------|---------------|
| `from` | number | Second where it starts. Default 0. |
| `seconds` | number | Duration, from 0.1 to 15. Default 4. |
| `fps` | integer | Frames per second, from 1 to 30. Default 10. |
| `base` | text | Only in a preset: the base package's file. |
| `format` | `gif` or `webp` | File type. `webp` keeps each color without loss; `gif` is the default. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid"}'
reiwand-ctl scene act my-scene shot '{"recording":{"from":0,"seconds":1,"fps":5}}'
```

### variant `flashes`

Measures how much the scene flashes. The result only lives in the process that measured it, so from the command line it is better to use the [`flashes`](queries.md#flashes) query with `"run": true`, which measures and answers in the same step.

| Argument | Type | What it means |
|-----------|------|---------------|
| `from` | number | Second where it starts. Default 0. |
| `seconds` | number | Duration, from 0.1 to 60. Default 8. |
| `fps` | integer | Frames per second, from 1 to 30. Default 15. |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"solid"}'
reiwand-ctl scene act my-scene shot '{"flashes":{"seconds":1,"fps":5}}'
```
