# Queries

### Particles

The first two queries are about a particle system. The system is given with `--layer ID` at the end of the command (without it the answer is `null`); the `layer` argument inside, if there is one, is the particle layer whose own drawing is looked at.

#### `particles`

Says everything that can be seen and touched in a particle system at a given second: where its control points and emitters are, what forces and colliders it has, how they are drawn, what data a `remap` rule can use and what the turbulence preview looks like. It is what the app draws on the canvas when editing a system.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Particle layer whose own drawing is looked at. With no value, the first one that draws the system. |
| `time` | number | Second of the scene. Default 0. |
| `sound` | number | Volume of the band that moves a turbulence's phase, from 0 to 1. Default 0. |

The answer has `points` (control points with their place on the canvas), `emitters` (for each emitter, the shape with its measures on the canvas and `max_per_step`), `colliders`, `pulls` (attractors, vortices and orbits), `keeps`, `boids`, `noises` (each `noise_field` force with its `index`), `operators`, `strands` (rope and trail), `children`, `remaps` (each rule with its curve), `inputs` and `steady_inputs` (the data a rule can read), `outputs` (what it can write), `domains` (the range of each piece of data), `render`, `scope` (which layers draw the system and how they differ), `cutout`, `orient`, `shaders` (particle effects that can be used), `fields` (the turbulence and the birth offset), `seed` and `sources` (layers they could be born from). If the system does not exist, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene ask my-scene particles '{"layer":"sparks","time":1.5,"sound":0.5}' --layer sparks
```

#### `budget`

Says how much a particle system costs: it simulates the system up to a given second at a quality level and counts how many particles there are and how much space they take on each kind of machine. In the app, the system's cost panel.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Particle layer whose own drawing is looked at. With no value, the first one that draws the system. |
| `time` | number | Second up to which it is simulated, from 0 to 10. Default 0. |
| `quality` | text | Quality level: `q0` (the lowest) to `q3` (the highest, default). |

The answer has `capacity`, `alive` (live particles at that second), `slots` (slots it reserves), `tiers` (for each kind of machine, from `t0` to `t3`, the slots on the graphics card and on the processor), `levels` (for each quality, the share of particles that are born, flock neighbors, trail steps and rope subdivisions), `boids`, `particles` (up to 300 particles as `[x, y, number]` in canvas pixels), `rope` and `trails`. If the system does not exist, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene ask my-scene budget '{"layer":"sparks","time":1.5,"quality":"q2"}' --layer sparks
```

#### `swarm_light`

Says how light affects a system's particles: whether they are lit, which normal map they use, whether they refract and what stays at each quality level. In the app, the particle system's light panel.

| Argument | Type | What it means |
|---|---|---|
| `system` | text | Id of the system. It can also be given with `--layer ID`. |
| `layer` | text | Particle layer whose own drawing is looked at. |

The answer has `lit` (yes/no), `normal`, `refract` (the refraction or `null`), `pictures` (package images that can serve as the map), `sides` (an embedded image of a sphere seen from the front and from behind with that light), `field`, `size`, `shift` (how far the refraction deflects, in pixels), `levels` (per quality, how many lights and what refraction) and `phone` (what remains on a phone). If the system does not exist, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"particles","id":"sparks"}'
reiwand-ctl scene ask my-scene swarm_light '{"system":"sparks"}'
```

### Lights and ambient light

#### `lights`

Lists the scene's lights at a given second, with their handles, the box that is redrawn when they move and how many remain on each kind of machine. In the app, the **Lights** panel.

| Argument | Type | What it means |
|---|---|---|
| `time` | number | Second of the scene. Default 0. |

The answer has `count`, `max` (12), `time`, `solo` and `muted` (the preview's solo and muted lights), `tiers` (for each kind of machine, from `t0` to `t3`: `cap` lights it lights, `kept` those that remain and `left_out` those that are left over) and `lights`. Each light has `id`, `pointer`, `type`, `at` (its place on the canvas), `aim` (where it points, if it points), `angle`, `color`, `intensity`, `radius`, `z`, `tilt`, `cone` (on spots), `box` (the box that is redrawn, `[x, y, width, height]`), `radius_knob`, `z_knob` and `tilt_knob` (where the handles are), `texture`, `volumetric`, `muted` and `solo`.

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene ask my-scene lights '{"time":0}'
```

#### `falloff`

Draws how a light fades with distance: the intensity curve from the light to where its radius ends. In the app, **Light against the distance**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the light. It can also be given with `--layer ID`. |
| `time` | number | Second of the scene. Default 0. |

The answer has `curve` (`points`, a list of `[distance, intensity]`, and the `x` and `y` bounds to draw it), `intensity`, `radius` and `marks`, distances worth marking (for example, where the cone starts to fade). If the layer is not a light, the answer is `null`.

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene ask my-scene falloff '{"layer":"spot","time":0}'
```

#### `lighting`

Says how much the scene's lighting costs at a given second and quality level: how many lights work, over how many layers and how many pixels, and how much air and refraction there is. In the app, the summary of the **Lights** panel.

| Argument | Type | What it means |
|---|---|---|
| `time` | number | Second of the scene. Default 0. |
| `quality` | text | Quality level: `q0` to `q3`. Default `q3`. |

The answer has `levels` (one row per quality with `lights`, `lit_layers`, `lit_particles`, `shaded` — lit pixels —, `per_frame`, `baked` — `true` if the light is computed just once at the start —, `air`, `air_taps`, `shaft_copy` and `refract`), `lit_layers` and `lit_systems` (what is lit), `animated`, `volumetric` (how many lights light the air), `tiers`, `phone`, `quality` and, for 3D models, `models`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene lighting '{"layer":"image","roughness":0.3}'
reiwand-ctl scene act my-scene light '{"add":{"light":"point","id":"point"}}'
reiwand-ctl scene ask my-scene lighting '{"time":0,"quality":"q2"}'
```

#### `air`

Says how the lit air of a point or spot light looks and how much it costs. In the app, **Light the air**.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the light. It can also be given with `--layer ID`. |
| `time` | number | Second of the scene. Default 0. |

If the light has air, the answer has `on` (`true`), `density`, `samples`, `shafts`, `box` (the zone it covers), `curve` (how it fades with distance), `shaded` (the same curve with the shafts, only if `shafts` is greater than 0), `marks`, `levels` (per quality, whether there is air, at what fraction of the size and with how many samples), `phone` (the same on a phone), `taps` (reads per frame at `q1`, `q2` and `q3`) and `fits`. Without air, it only has `on` as `false` and `fits`, which says whether that kind of light can have air (only point and spot lights). If the layer is not a light, the answer is `null`.

```sh
reiwand-ctl scene act my-scene light '{"add":{"light":"point","id":"point"}}'
reiwand-ctl scene act my-scene light '{"volumetric":{"layer":"point","density":0.3}}'
reiwand-ctl scene ask my-scene air '{"layer":"point","time":0}'
```

#### `projection`

Says what a spot projects and where the image lands on the canvas. In the app, the light's **Projects** section.

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the spot. It can also be given with `--layer ID`. |
| `time` | number | Second of the scene. Default 0. |

The answer has `current` (what it projects, or `null`), `kind` (`picture`, `video` or `layer`), `landed` (`true` if the image lands inside the canvas), `quad` (the four corners of the projected image, in canvas pixels), `image` (the embedded image), `spot` (`true` if the light is a spot) and `pictures`, `videos` and `layers`, what can be projected. If the layer is not a light, the answer is `null`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"image","file":"picture.png"}'
reiwand-ctl scene act my-scene light '{"add":{"light":"spot","id":"spot"}}'
reiwand-ctl scene act my-scene light '{"projection":{"layer":"spot","asset":"picture"}}'
reiwand-ctl scene ask my-scene projection '{"layer":"spot","time":0}'
```

#### `hemisphere`

Shows the scene's ambient light and sky light as a sphere that goes from one color to the other. It takes no arguments. In the app, the sphere next to **Light around** and **Light from above**.

The answer has `ambient` and `sky` (colors as `[r, g, b, a]`, from 0 to 1; black if there is none), `size` and `sphere`, an image embedded as `data:image/png;base64,...` text.

```sh
reiwand-ctl scene act my-scene ambient '{"ambient":"#202040","sky":"#8090ff"}'
reiwand-ctl scene ask my-scene hemisphere '{}'
```

### Several screens

#### `spread`

Says what part of the canvas each screen would show if the scene were spread over several screens at once, with simulated screens. It is for checking the spread without having the screens.

| Argument | Type | What it means |
|---|---|---|
| `layout` | text | (required) The arrangement: `dual` (two screens in a row), `triple` (three in a row), `vertical` (two one above the other) or `mine` (yours, with `monitors`). |
| `bezel` | number | Millimeters of bezel on each screen, which separate the images. Default 0. |
| `screen` | text | Size of the simulated screens: `"WIDTHxHEIGHT"` in millimeters (`"527x296"`) or a diagonal such as `"27in"` or `"34in@21:9"`. Default 527 by 296 millimeters. From 50 to 3000 mm per side. |
| `monitors` | list of objects | With `mine`, the screens: `{"id": "name", "mm": [x, y, width, height]}`, with the position and size in millimeters. Between 1 and the maximum a layout accepts. |

The answer has `spread` (`true` if the package asks to be spread over all the screens), `canvas` (the canvas size), `screens` (the screens used), `bounds` (the rectangle all of them occupy, in millimeters) and `parts`, a list of `{id, rect}` with the `[x, y, width, height]` rectangle of the canvas, in pixels, that each screen shows. If something is not valid, it brings `error` and an empty `parts`.

```sh
reiwand-ctl scene ask my-scene spread '{"layout":"dual"}'
reiwand-ctl scene ask my-scene spread '{"layout":"vertical","bezel":10,"screen":"27in@21:9"}'
reiwand-ctl scene ask my-scene spread '{"layout":"mine","monitors":[{"id":"left","mm":[0,0,600,340]},{"id":"right","mm":[620,0,527,296]}]}'
```

### Sound

**What a sound layer is like.** Sound layers are edited with `add_layer` of kind `sound` and with `set` on their fields; they have no actions of their own. `add_layer` with `"kind":"sound"` and `"file":"sound.ogg"` creates the layer, imports the file and gives it the id `sound`. Its fields are: `sound` (the id of a package sound) or `sounds` (a list of 1 to 16 sounds; a layer takes one of the two), `loop` (whether it repeats without gaps; yes by default), `play` (whether it plays; yes by default), `volume` (from 0 to 1), `pan` (from −1 left to 1 right), `fade_in` and `fade_out` (seconds of rise and fall, from 0 to 60), `mode` (with `sounds`: `sequence` or `random`), `pause` (`[min, max]` seconds of silence between sounds, from 0 to 600), `stream` (yes for long sounds) and `play_on` (a trigger, such as `start`, `beat` or `event:name`, that starts the sound each time it is met). `volume`, `pan` and `play` can follow a user property or keys; they cannot follow the sound that is heard (`validate` rejects it). Hiding the layer with `hide` mutes it.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"sound","file":"sound.ogg","id":"wind"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/loop","value":false}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/fade_in","value":2}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/volume","value":0.6}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/pan","value":-0.3}'
```

#### `sounds`

Lists the scene's sound layers and the package's sounds. It takes no arguments. In the app, **Sounds of the scene**.

The answer has `layers`, with one row per layer: `id`, `pointer`, `sounds` (the sounds it uses), `mode`, `loop`, `stream`, `play_on`, `fade_in`, `fade_out`, `volume` (the number, or `null` if keys or a property move it) and `muted` (yes/no); and `assets`, the package's sounds with `name`, `path` and `bytes`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"sound","file":"sound.ogg","id":"wind"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/play_on","value":"start"}'
reiwand-ctl scene ask my-scene sounds '{}'
```

#### `wave`

Computes the waveform of a package sound: the highest level of each stretch, to draw it. It is saved in a cache and the second time it answers at once.

| Argument | Type | What it means |
|---|---|---|
| `asset` | text | (required) Id of a package sound. |
| `buckets` | integer | Into how many stretches the sound is divided, from 1 to 2048. Default 256. |

The answer has `seconds` (the duration), `channels` and `peaks`, a list of `buckets` numbers from 0 to 1. If the sound does not exist or cannot be read, it brings `error`.

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"sound","file":"sound.ogg"}'
reiwand-ctl scene ask my-scene wave '{"asset":"sound","buckets":8}'
```

#### `meter`

Says what a sound binding reacts to and what value it gives at a given sound level: which bands are marked, how their levels come out and what value results before and after shaping. A sound binding is the value `{"audio": ...}` that is put on a layer's or effect's property (for example, the opacity), and that is shown as **Follow the sound**. The sound is analyzed in 64 bands per channel, from bass to treble, and each level goes from 0 to 1.

| Argument | Type | What it means |
|---|---|---|
| `source` | object | (required) The binding: `audio` is `"bass"`, `"mid"`, `"treble"`, `"level"` (volume of everything), `"beat"` (1 on each beat), the number of a band (`3`) or a range (`[0, 3]`). It accepts `of` (how many bands the grid has: 16, 32 or 64, default 64), `reduce` (`mean`, the average, or `max`, the maximum, for a range), `channel` (`left`, `right` or the average, default), and the shaping: `bounds`, `exponent`, `invert`, `scale`, `offset` and `clamp`. The `smooth` field is also valid in a binding, but this query does not apply it: to see its effect use `response`. |
| `reading` | object | What was heard: `listening` (yes/no; without `true`, the answer only brings the marked bands), `raw` (the bands' levels: one list for the left channel and another for the right, of up to 64 numbers each), `loud` (the same, measured in absolute value; it is used if the scene sets `audio.gain` to `fixed`), `loudness`, `beat` (yes/no), `silent` (yes/no) and `granted` (yes/no; default `true`). |
| `of` | integer | How many bands the answer's grid has: 16, 32 or 64. Default 64. |

The answer has `granted`, `listening`, `of` and `marked` (`[first, last]` marked band in that grid, or `null` if the binding is the volume or the beat). With `listening` as `true` it adds `bands` (the levels folded to `of` bands), `levels` (`bass`, `mid`, `treble`, `level` and `beat`), `silent`, `raw` (the value before shaping) and `value` (the value with the shaping applied: `bounds`, `exponent`, `invert`, `scale`, `offset` and `clamp`). If the binding is badly written or `of` is not 16, 32 or 64, it brings `error`.

```sh
reiwand-ctl scene ask my-scene meter '{"source":{"audio":[0,3], "kind": "relative", "unit": "db","of":16},"reading":{"listening":true,"raw":[[0.5,0.2,0.1,0.9],[0.5,0.2,0.1,0.9]],"loudness":0.5},"of":16}'
reiwand-ctl scene ask my-scene meter '{"source":{"audio":"bass", "kind": "relative", "unit": "db","scale":2,"clamp":[0,1]},"reading":{"listening":false}}'
```

#### `response`

Draws how a sound binding responds with `smooth`: it rises to the maximum and returns to zero, and how long it takes to settle in each direction. It is for choosing `smooth` without trying it by ear.

| Argument | Type | What it means |
|---|---|---|
| `smooth` | object or number | (required) `{"up": seconds, "down": seconds}` (how long it takes to rise and to fall, from 0 to 60), or a single number for both directions. |

The answer has `up`, `down`, `settles` (the seconds it takes to settle when rising and when falling), `rises` and `falls` (the seconds at which the sound jumps up and down in the drawing), `duration` and `points`, 96 `[second, value]` pairs with the curve from 0 to 1. If the times are not valid, it brings `error`.

```sh
reiwand-ctl scene ask my-scene response '{"smooth":{"up":0.05,"down":0.4}}'
reiwand-ctl scene ask my-scene response '{"smooth":0.3}'
```

### Audio bands

#### `bands`

Answers with the division of the sound into bands: each one's frequency range and which bands are the bass, mids and treble. It is for choosing which bands a sound connection listens to. In the app, the band selector.

| Argument | Type | What it means |
|---|---|---|
| `of` | integer | Into how many bands it is divided: 16, 32 or 64. Default 16. |

The answer has `of`, `bands` (each band with `index`, `from` and `to` in hertz), `named` (the range of `bass`, `mid` and `treble`), `levels` (the named levels) and `pointer` (the pointer addresses that can be read). With `pick` or `direction` in place of `of`, it answers with what that choice reads. If something is not valid, it brings `error`.

```sh
reiwand-ctl scene ask my-scene bands '{"of":16}'
```
