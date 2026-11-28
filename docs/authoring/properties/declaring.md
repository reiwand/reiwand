# Declaring properties

On this page: declaring and using a property, the common fields and the types.

## Declare and use a property

In the manifest:

```json
"color_disc": {
  "alpha": false, "default": "#ffb347", "group": "look",
  "label": "Disc color", "order": 1, "type": "color",
  "enabled_if": {}, "quality": {}, "visible_if": {}
}
```

In the scene, any value can follow it with `{ "prop": "color_disc" }`:

```json
    {
      "blend_space": "canvas", "id": "disc", "type": "shape",
      "shape": { "ellipse": { "size": [300, 300] } },
      "fill": { "prop": "color_disc", "invert": false },
      "opacity": { "prop": "opacity", "scale": 0.01, "invert": false },
      "transform": {
        "position": [480, 540],
        "scale": {
          "timeline": "flash",
          "keys": [[0, [1, 1]], [0.3, [1.5, 1.5], "out_cubic"], [1, [1, 1], "in_out_sine"]],
          "invert": false
        },
        "origin": "anchor", "rotation": 0.0, "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "parallax": [0, 0],
      "solid": false, "visible": true
    }
```

What value the property gives depends on its type (see below), and the scene takes it already
clamped and rounded. Only a property that stores a value can be followed; a `button`, a `group`, a
`label` and a `divider` do not store one.

A property id is a letter followed by up to 63 letters, digits or `_`.

From the terminal:

```
reiwand-ctl scene act MY-FOLDER add_property '{"kind":"color","id":"color_disc","label":"Disc color"}'
reiwand-ctl scene act MY-FOLDER property '{"id":"color_disc","default":"#ffb347"}'
reiwand-ctl scene act MY-FOLDER set '{"pointer":"/scene/layers/2/fill","value":{"prop":"color_disc"}}'
```

In the editor, the *User properties* panel lists the properties in the order of the form;
*Add a property* creates one of any type, and the inspector edits each field of its type.
Links are made with the diamond of each value, which offers *Follow a user property*
(see [editor.md](../editor.md)).

## Common fields

All properties can carry:

| Field | What it does |
|-------|--------------|
| `type` | The type. Required. |
| `label` | The name that is shown. If missing, the id is shown. |
| `help` | A longer help text. |
| `group` | The id of a `group` property it goes into. |
| `order` | A number: properties are sorted by it and then by id. |
| `icon` | An icon, from the list below. |
| `visible_if` | Hides the property while the condition is not met. |
| `enabled_if` | Turns it off (grayed out) while the condition is not met. |
| `quality` | Values per quality level, below. |

The conditions of a group apply to all its members.

## The types

| `type` | Fields | What it stores |
|--------|--------|----------------|
| `color` | `default`, `alpha` | A color `#rrggbb` or, with `alpha`, `#rrggbbaa`. |
| `slider` | `min`, `max`, `default`, `step`, `decimals` (0 to 6), `integer`, `unit`, `editable`, `log` | A number, clamped to the range, snapped to the step and rounded to the decimals. |
| `bool` | `default` | `true` or `false`. |
| `combo` | `options`, `default` | One of the values of the options. |
| `text` | `default`, `max_len` (1 to 1024), `multiline`, `secret` | A text. |
| `file` | `accept` (`image`, `video`), `default_asset` | The path of a file the user chooses, or `null`. |
| `folder` | `accept`, `pick`, `every`, `mode`, `max_files`, `recursive` | The path of a folder the user chooses, or `null`. |
| `shortcut` | | What the user chose to open, or `null`. |
| `button` | | Nothing: pressing it is an event. |
| `group` | `collapsed` | Nothing: lists below it the properties that name it. |
| `label`, `divider` | | Nothing: a line of text, or a rule with optional text. |

### Slider

```json
"opacity": {
  "default": 100, "editable": true, "group": "look",
  "help": "How solid the disc looks.",
  "integer": true, "label": "Disc opacity", "max": 100,
  "min": 10, "order": 2, "quality": { "q0": 50, "q1": 70 }, "step": 5, "type": "slider", "unit": "%",
  "decimals": null, "enabled_if": {}, "log": false, "visible_if": {}
}
```

Without `step`, the step comes from `decimals`, and if that is missing, from the decimals of `min`,
`max` and `step` (the largest), or two if there are none. With `integer` there are no decimals.
`min` and `max` are always accepted, even if they do not fall on the step grid, and no rounding
takes a value out of the range. `editable` adds a box to type the number, `unit` sets a unit (`%`,
`px`...) and `log` spreads the range on a logarithmic scale (it needs `min` greater than 0). A
`slider` can be followed in the scene with `scale`, as in `{ "prop": "opacity", "scale": 0.01 }`.

### Choice (`combo`)

`options` is a list of values, or of `{ "value", "label", "icon" }`, up to 64, all of the same
type (texts, numbers or `true` and `false`). An option with an empty label shows its value.
In the scene, `map` chooses a value according to the option: `{ "prop": "style", "map": { "day":
"#ffe08a", "night": "#8fb4ff" } }`. In image and video layers, effect texture parameters, mask
textures and particle textures, `map` switches the resource, so a property picks between variants
of your scene.

### Text

`default`, `max_len`, `multiline` (a multi-line box) and `secret`. A secret text has no prefilled
value, is stored in the system keyring (on the phone, sealed by the Android keystore) and never in
the configuration, the logs, the presets or the exported settings. A scene reads it as an empty
text: only content that can use the network receives it.

### File and folder

A `file` that the user chooses replaces the resource that `default_asset` names wherever the scene
uses it (image and video layers, masks, effect and particle textures), and it shows wherever
`{ "prop": "id" }` names it, with or without `default_asset`. With `accept: ["video"]` a video
plays in image and video layers, without sound and looping. A change only replaces the textures it
touches: the clock, the timelines, the particles and the scripts carry on. `reiwand-ctl pack check`
warns if the images of the variants of one place do not have the same aspect ratio.

A `folder` shows one of its images (or videos, with `accept`) in an image layer linked with
`{ "prop": "id" }`:

> In these tables, *Prefill* is the value the editor and the templates write for you. The file still has to contain every field: the loader refuses a missing one and never fills it in (see `../specs/explicit-values.md`).

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `pick` | `random` | `random`, `sequence` | At random or in order. |
| `every` | 60 | 10 to 86400 s | How often it changes, while the scene is in view. |
| `mode` | `one` | `one`, `all` | `all` lets web pages list all the files. |
| `max_files` | 1000 | 1 to 1000 | How many files it reads, in name order. |
| `recursive` | 0 | 0 to 2 | Levels of subfolders. |

Only the folder itself is read (and its subfolders down to that level), without following links
that leave it or reading hidden files, and nothing is read while the scene is not in view. The user
chooses files and folders with the file picker; presets do not set them and they do not travel with
exported settings.

### Shortcut

`shortcut` stores what the user chose to open from the wallpaper: an `https://` address (up to
2048 characters), a file, a folder or an application by its desktop id, never a command. Only the
user sets it, never the author or a preset. The content never sees the destination, only `true` if
there is one; a script opens it with `openShortcut(property)` inside a real click on a layer (see
[scripts.md](../scripts.md)). The package must have the click permission (see
[package.md](../package.md)).

### Button, group, label and divider

A `button` stores nothing; pressing it starts the timelines whose `trigger` is `prop:<id>` and
shows it to scripts for one frame. It cannot be pressed while its `enabled_if`, or that of its
group, is not met. A `group` gathers properties (groups do not nest) and with `collapsed` it
appears folded. A `label` is a line of text and a `divider` a rule with optional text.

```json
  "timelines": [
    {
      "id": "flash", "duration": 1, "loop": "once", "autoplay": false, "trigger": "prop:flash",
      "delay": 0.0, "events": [], "fps": null, "frames": null, "speed": 1.0, "wrap": false
    }
  ]
```

The `flash` button of the example fires that timeline.
