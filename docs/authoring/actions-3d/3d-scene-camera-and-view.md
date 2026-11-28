# Camera, view, lights and shadows

On this page: 3D scene: camera and view, 3D scene: lights, 3D scene: shadows.

## 3D scene: camera and view

There are two different things. The **camera** is the scene's: the one that whoever has the wallpaper will see. The **view** is only the way of looking while you edit: turning, zooming or panning the view does not change the scene's camera, and is never seen in the finished wallpaper.

### Variant camera: the scene's camera

Changes the scene's 3D camera. Only the fields you give are touched. In the app it is the **3D camera** panel.

| Argument | Type | What it means |
|---|---|---|
| `fov` | number | Field of view in degrees, from 1 to 179 (default 60). |
| `near` | number | Nearest distance that is drawn, from 0.001 to 1000 (default 0.1). |
| `far` | number | Farthest distance, greater than `near` and up to 1,000,000 (default 1000). |
| `eye` | list of 3 numbers | Where the camera is. Default `[0, 0, 5]`. |
| `center` | list of 3 numbers | Point it looks at. Default `[0, 0, 0]`. |
| `up` | list of 3 numbers | Which direction is "up"; it cannot be `[0, 0, 0]`. Default `[0, 1, 0]`. |
| `roll` | number | Rotation of the camera around its line of sight, from −180 to 180 degrees, counted from the vertical. It changes `up`. |
| `from_view` | yes/no | `true` puts the camera where the authoring view is looking now. Default `false`. In the app it is **Set the camera from the view**. |
| `clear` | yes/no | `true` erases the camera and goes back to the default one. Default `false`. |

```sh
reiwand-ctl scene act my-scene scene3d '{"camera":{"fov":45,"eye":[0,2,8],"center":[0,1,0]}}'
reiwand-ctl scene act my-scene scene3d '{"camera":{"roll":10,"near":0.5,"far":200}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"orbit":[30,10]}}'
reiwand-ctl scene act my-scene scene3d '{"camera":{"from_view":true}}'
reiwand-ctl scene act my-scene scene3d '{"camera":{"clear":true}}'
```

### Variant view: the authoring view

Turns, pans, zooms or frames the view you use to look at the scene while you edit. The values are changes: they are added to what is already there. The first time you turn or move the view, it stops being the scene's camera and the camera handles appear.

| Argument | Type | What it means |
|---|---|---|
| `orbit` | list of 2 numbers | Rotation around the point being looked at: `[horizontal, vertical]` in degrees. The vertical tilt is limited to 89 degrees. |
| `pan` | list of 2 numbers | Shift `[x, y]` in canvas pixels. |
| `zoom` | number | Factor greater than 0: 2 zooms in twice as much, 0.5 zooms out by half. |
| `frame` | text | Id of a visible model layer: the view frames it whole. In the app it is **Look at it in the authoring view**. |
| `through` | yes/no | `true` looks through the scene's camera (**Look through the camera**), `false` uses the free view (**Back to the camera view** does the opposite). |
| `reset` | yes/no | `true` erases the view and goes back to the camera. Default `false`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"orbit":[30,10],"pan":[50,0],"zoom":1.5}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"frame":"cube"}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"through":true}}'
reiwand-ctl scene act my-scene scene3d '{"view":{"reset":true}}'
```

### Variant tones: bright color and tone map

Turns on high dynamic range color (HDR), which keeps colors brighter than white, and chooses how they are compressed to display them. In the app it is the **Tone map** panel.

| Argument | Type | What it means |
|---|---|---|
| `hdr` | yes/no | `true` keeps colors brighter than white. `false` turns HDR off and the tone map too. |
| `tonemap` | text | `none`, `aces` (filmic), `agx` (soft) or `srgb_to_linear` (the final colors, glow included, through the sRGB curve). Only with HDR on. |
| `split` | number | From 0 to 1: shows in the authoring view the image without the tone map to the left of that point and with it to the right (**Before and after**). 0 removes it. |

```sh
reiwand-ctl scene act my-scene scene3d '{"tones":{"hdr":true,"tonemap":"aces"}}'
reiwand-ctl scene act my-scene scene3d '{"tones":{"split":0.5}}'
reiwand-ctl scene act my-scene scene3d '{"tones":{"hdr":false}}'
```

## 3D scene: lights

### Variant light: put a light in 3D

A normal light layer lights in the plane. This variant places it among the models, at a 3D point, and gives it a point it aims at (spotlights and directional lights). In the app it is each light's switch in the **Lights in 3D** panel; its points can also be dragged with [`grab`](3d-scene-camera-paths.md#variant-grab-drag-a-handle).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the light layer. |
| `place` | yes/no | `true` puts it in 3D right where it appears now (and, except for point lights, aiming at where it lights now). Default `false`. |
| `position` | list of 3 numbers | Exact position, up to a million units. |
| `aim` | list of 3 numbers | Point it aims at, up to a million units. |
| `flat` | yes/no | `true` removes the position and the aim point: it goes back to the plane. Default `false`. |

```sh
reiwand-ctl scene act my-scene import '{"file":"figure.glb"}'
reiwand-ctl scene act my-scene scene3d '{"model":{"add":{"asset":"figure","id":"cube"}}}'
reiwand-ctl scene act my-scene add_layer '{"kind":"light","id":"lamp"}'
reiwand-ctl scene act my-scene scene3d '{"light":{"layer":"lamp","place":true}}'
reiwand-ctl scene act my-scene scene3d '{"light":{"layer":"lamp","position":[2,3,4],"aim":[0,0,0]}}'
reiwand-ctl scene act my-scene scene3d '{"light":{"layer":"lamp","flat":true}}'
```

## 3D scene: shadows

### Variant shadows: a light's shadows

Makes a point light, a spotlight or a directional light cast shadows on the models. In the app it is the **Shadows** panel. In a scene at most 2 lights cast shadows, and more modest machines draw fewer (the [`shadows3d`](queries.md#shadows3d) query says how many per level).

| Argument | Type | What it means |
|---|---|---|
| `layer` | text | Id of the light layer. |
| `on` | yes/no | `true` turns the shadow on. `false` turns it off and erases its size and its bias. |
| `resolution` | integer | Side of the shadow map in pixels, from 256 to 4096. |
| `automatic` | yes/no | `true` erases the chosen size and lets the program decide (**Automatic**). Default `false`. |
| `bias` | number | Bias from 0 to 1 (**Bias**). |

```sh
reiwand-ctl scene act my-scene add_layer '{"kind":"light","id":"lamp"}'
reiwand-ctl scene act my-scene scene3d '{"shadows":{"layer":"lamp","on":true,"resolution":1024,"bias":0.1}}'
reiwand-ctl scene act my-scene scene3d '{"shadows":{"layer":"lamp","automatic":true}}'
reiwand-ctl scene act my-scene scene3d '{"shadows":{"layer":"lamp","on":false}}'
```
