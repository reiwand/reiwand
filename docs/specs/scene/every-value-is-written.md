# Every value is written

A scene carries every value: the loader refuses a file that lacks a field and names its path (`"/layers/2/blur"
is missing. Reiwand does not fill in values: write it.`). Nothing is
completed silently and the engine keeps no hidden value for a field of the format.

- The value that the tables below call «prefilled» is the one the editor, `reiwand-ctl scene new` and
  `scene act` write when a field is created; it is a suggestion that is saved in the file, not a rule of the
  loader. A field with a range, a unit or a word lists them in the table; a field that follows another thing
  (the anchor of a layer, the canvas) carries an explicit word (`"anchor"`, `"canvas"`, `"auto"`), not a missing
  value.
- A block of an optional feature that the scene does not use (a mask, a trail, `camera3d`, `audio`, the effects of
  a layer…) may be absent from the file, or written as `null`: both mean the same, and the editor does not write
  it. A block that is present carries all its fields. A field whose `null` has a meaning of its own (the end of a
  playback, the frame cap) is always written.
- Where a table says «`null`» for an unused field, read it as the value of an unused block; where it says
  «without it», read «when the block is absent or `null`».
- The file classes of every field (block or value) are `crates/reiwand-scene/src/classes.tsv`, and the prefilled
  value of each, `crates/reiwand-scene/src/prefill.json`. The whole examples are the ones in
  `docs/authoring/examples`; the snippets of this page show the fields that matter for each point and are fragments.
- `schema` is `reiwand.scene/2`. A scene with any other schema is refused (`unsupported schema`); no program
  reads or converts an older one.

Fragment of a scene (a whole one has every field of its blocks; see `docs/authoring/examples`):

```json
{
  "schema": "reiwand.scene/2",
  "canvas": { "size": [1920, 1080], "fit": "fill", "background": "#0b1020", "blend_space": "linear" },
  "clock": { "max_fps": 60, "poster_time": 3.0, "seed": 42 },
  "assets": { "sky": { "path": "assets/sky.png" }, "mono": { "path": "assets/mono.ttf" } },
  "layers": [
    { "blend_space": "canvas", "id": "sky", "type": "image", "image": { "prop": "style", "map": { "night": "sky" } },
      "effects": [{ "effect": "waves", "mask": { "texture": "lake" }, "params": { "amplitude": 4 } }] },
    { "blend_space": "canvas", "id": "clock", "type": "text", "font": "mono", "size": 96, "text": { "clock": "HH:mm" },
      "anchor": "bottom_right", "transform": { "position": [-64, -64] },
      "opacity": { "timeline": "intro", "keys": [[0, 0, "out_cubic"], [1.5, 1]] } }
  ],
  "timelines": [{ "id": "intro", "duration": 1.5, "loop": "once" }]
}
```
