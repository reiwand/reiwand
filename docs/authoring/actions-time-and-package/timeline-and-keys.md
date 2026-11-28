# Timeline and keys

On this page: Timeline and keys.

## Actions

## Timeline and keys

A key is a value set at a second. When a field has keys, the scene computes what it is worth between one key and the next. In the document, an animated field stops being a number and becomes `{"keys": [[second, value], ...]}`; a key can carry a third element with its easing. Keys follow the scene's clock, or a timeline's if the field says `"timeline"`.

### `keyframe`

Adds, changes, moves or removes a key of an animatable value (number, list of numbers, color, switch or text). With the first key, the field becomes animated. In the app these are the timeline and the key dialog.

| Argument | Type | What it means |
|-----------|------|---------------|
| `pointer` | text (required) | Path of the value, for example `/scene/layers/0/opacity`. |
| `time` | number (required) | Second of the key. |
| `value` | any JSON value | Value of the key. If missing, the value the field has at that second is used. |
| `ease` | text or list of 4 numbers | Easing from this key to the next: `linear`, `step`, `hold`, or one of the curves `in_`, `out_` or `in_out_` followed by `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` or `bounce` (for example `in_out_sine`); or `[x1, y1, x2, y2]`, a Bézier curve. |
| `timeline` | text | Id of a timeline that already exists (see [`timeline`](#timeline)): the animation follows that timeline and not the scene's clock. |
| `remove` | yes/no | `true` removes the key at `time`. Default `false`. If you remove the last one, the field goes back to being a fixed value. |
| `from` | number | Moves the key that is at second `from` to second `time`, with its value and easing. It fails if there is already another key at `time`. |
| `base` | any JSON value | Makes the keys relative: they are added to this value (a number, a list of numbers or a color). |
| `absolute` | yes/no | `true` removes `base` and leaves the values as they are. Default `false`. |
| `in_handle` | list of 2 numbers | Bézier handle before the key: `[seconds, value]`. It replaces the easing. |
| `out_handle` | list of 2 numbers | Bézier handle after the key. It replaces the easing. |
| `straighten` | yes/no | `true` straightens the curve leaving this key (removes its outgoing handle or its easing and the next one's incoming handle). It goes alone, with `pointer` and `time`. Default `false`. |

A handle cannot reach farther than the neighboring key. To see the result of the curves, look at the [`curves`](queries.md#curves) query.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1,"ease":"in_out_sine"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":4,"value":0.2,"ease":[0.4,0,0.2,1]}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"in_handle":[0.5,0.1],"out_handle":[0.5,0]}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"straighten":true}'
```

To move a key from one second to another, remove it and work with relative keys:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":3,"from":2}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/opacity","time":3,"remove":true}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/transform/rotation","time":0,"value":0,"base":10}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/transform/rotation","time":1,"value":5,"absolute":true}'
```

To animate with a timeline of your own:

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png","id":"sky"}'
reiwand-ctl scene act my-scene timeline '{"id":"round","add":true,"duration":3}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/transform/position","time":0,"value":[0,0],"timeline":"round"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/0/transform/position","time":3,"value":[100,0],"timeline":"round"}'
```

### `timeline`

Adds, changes, renames or removes a timeline and its events. A timeline is a clock of the scene's own (with duration, repetition, speed and a trigger that starts it) that animated values can be tied to. It prints the id when adding it (up to 64 per scene). In the app, the **Timelines** menu and **Edit the timeline and its events…**.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the timeline. |
| `add` | yes/no | `true` creates a new timeline (5 s if you give no duration). Default `false`. |
| `remove` | yes/no | `true` removes it; everything it animated goes back to following the scene's clock. Default `false`. |
| `to` | text | New id. What follows it is updated. |
| `duration` | number | Duration in seconds. It cannot be combined with `frames`. |
| `frames` | integer | Duration in frames, at the speed `fps`; the action writes `duration` as `0` then. |
| `fps` | number | Frames per second for `frames`, from 1 to 240. |
| `loop` | text | `once`, `repeat` (repeats) or `pingpong` (goes back and forth). |
| `wrap` | yes/no | Only with `repeat`: when repeating, the last value joins with the first. |
| `speed` | number | Speed from -100 to 100; 1 is normal. |
| `delay` | number | Wait in seconds before starting. |
| `autoplay` | yes/no | Whether it starts by itself when the scene loads; only without a `trigger` (`start` always plays, any other trigger waits). |
| `trigger` | text | What starts it: `start`, `beat`, `prop:ID` (a user property), `event:NAME` (an event of another timeline or of a clip), `click:LAYER`, `enter:LAYER` or `leave:LAYER`. Empty text removes it. |
| `add_events` | list of objects | New events: `{"t": second, "name": name}`; the name is 1 to 32 lowercase letters, digits or `_`. Up to 64, within the duration. |
| `drop_events` | list of texts | Events that are removed, by name (all those with that name) or `name@second` (only the one at that second). |
| `clear_events` | yes/no | `true` removes all the events first of all. Default `false`. |

```sh
reiwand-ctl scene act my-scene timeline '{"id":"round","add":true,"duration":3}'
reiwand-ctl scene act my-scene timeline '{"id":"round","loop":"pingpong","speed":2,"delay":0.5,"trigger":"start","add_events":[{"t":1,"name":"middle"},{"t":2,"name":"almost"}]}'
reiwand-ctl scene act my-scene timeline '{"id":"round","drop_events":["almost"]}'
reiwand-ctl scene act my-scene timeline '{"id":"round","clear_events":true}'
reiwand-ctl scene act my-scene timeline '{"id":"round","to":"spin"}'
reiwand-ctl scene act my-scene timeline '{"id":"spin","remove":true}'
```

```sh
reiwand-ctl scene act my-scene timeline '{"id":"short","add":true,"frames":60,"fps":30}'
reiwand-ctl scene act my-scene timeline '{"id":"short","loop":"repeat","wrap":true}'
```
