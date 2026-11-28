# Your first scene

You are going to make a scene with a background image that ripples gently, a sun that crosses the
screen, and a setting that lets the user pick the sun's color. Each step has two paths: the
`reiwand-ctl` command and where it is in the editor. The final result is the
[`examples/first-scene`](../examples/first-scene) folder.

Put `sky.png` in the folder where you are going to work: it can be any image of yours with that
name, or the one in the example folder, `examples/first-scene/assets/sky.png`. In the editor the
name does not matter: you will pick whatever file you have.

### 1. Create the scene

```
reiwand-ctl scene new my-scene --title "My first scene"
```

*Editor:* *Create* page, *Start a new one*, the *Empty scene* tile; below the tiles, *Change…* gives
it its title and folder, and *Create* makes it. The editor opens on the *Scene* layout: the canvas in
the middle, the *Inspector* and the *Layers* on the right, the *Timeline* below.

This creates the folder `my-scene` with a `manifest.json`, a `scene.json` with no layers, and an
`assets/` folder. The canvas is 1920 by 1080: that is the reference size you place everything
against, and the program adjusts it to each screen. The package gets the id
`org.example.my-scene` (the folder name); it is the one that commands like `input grant` use, and
you should change it before publishing.

### 2. An image layer

```
reiwand-ctl scene act my-scene add_layer '{"kind":"image","name":"Background","file":"sky.png"}'
reiwand-ctl scene act my-scene layer '{"transform":{"layer":"background","size":[1920,1080]}}'
```

*Editor:* *Add a layer* (the + at the head of the *Layers* panel) > *Layer from a file* > *Picture…*
and choose your image (you can also drop
the file onto the *Layers* list, where it says “Drop files to add them as layers”, if your file
manager allows dragging). The layer is born with the id `image`: right-click it in *Layers* (or press the
menu key on it), *Rename the layer*, and type `background`. Then, in the layer's inspector, scroll down past the middle to *Playback* and set *Size*
to 1920 by 1080 (before that it is 0 by 0, and the image already fills the canvas).

The first command copies the image into `assets/` as the asset `sky`, adds a layer that shows
it, and prints its id: `background`. Without `size`, an image is shown at its real size and centered,
which is almost never the canvas size: with `size` it is stretched to fill it, and you have to set
it on every image layer. Each layer has an id, made from its name, which the other commands use to
refer to it. Layers, assets, particle systems and timelines share their ids, so if the asset was
already named like the layer (`--file background.png --name Background`), the layer is called `background_2`:
always read the id the command prints (`cli.md`, “`scene act DIR add`”).

### 3. A layer that moves

First a shape, a yellow circle:

```
reiwand-ctl scene act my-scene add_layer '{"kind":"shape","name":"Sun"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/1/shape","value":{"ellipse":{"size":[220,220]}}}'
```

*Editor:* *Add a layer*, type *Shape*; the layer is born with the id `shape` and a rectangle: with
*Rename the layer* in its menu, call it `sun`. Further down in the inspector, under *Shape*,
choose *Ellipse* (it starts at 220 by 220; change its size if you like) and leave the fill blue for
now: you change it in step 5.

Then a timeline that lasts 12 seconds and goes back and forth, and two keyframes for the sun's
position, one at the start and one at the end:

```
reiwand-ctl scene act my-scene timeline '{"id":"lap","add":true,"duration":12,"loop":"pingpong"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/1/transform/position","time":0,"value":[260,820],"ease":"in_out_sine","timeline":"lap"}'
reiwand-ctl scene act my-scene keyframe '{"pointer":"/scene/layers/1/transform/position","time":12,"value":[1660,260],"timeline":"lap"}'
```

*Editor:* in the *Timeline* panel, *Change…* > *Add a timeline* (the menu closes without further
notice; when you open it again you will see `timeline_1 · 5 s`). That timeline is born with the id
`timeline_1`: in its submenu, *Rename…* and call it `lap`. In the same submenu, *Edit the
timeline and its events…* opens a dialog: set *Seconds* to 12 and choose *Back and forth* in
*Plays*. With the playhead at 0, click the diamond of the sun layer's *Position* (*Keyframe here*):
the untouched position is the center of the canvas, so the keyframe is born there. Only then does a
*sun · Position* row appear in the time strip, with a dropdown that says *Scene clock*: choose
`lap` in it. Type 260 and 820 in *Position*, or drag the sun to the bottom left. Move the
playhead to 12 and type 1660 and 260, or drag the sun to the top right.

Layers are counted from 0 starting at the bottom, which is why the sun, the second one, is
`/scene/layers/1`. To see what is anywhere: `reiwand-ctl scene show my-scene /scene/layers/1`. The
pointer starts with `/scene` (what is in `scene.json`) or with `/manifest` (what is in
`manifest.json`); `/layers` alone does not exist.

The position is a point on the canvas in pixels with the origin at the top left. `pingpong` is the
value of `--loop` (and of `loop` in `scene.json`) that makes the timeline go back and forth
endlessly, and it is what the editor calls *Back and forth*; `in_out_sine` eases the start of the
trip.

### 4. An effect

An effect changes how a layer looks. `waves` makes the sky ripple:

```
reiwand-ctl scene act my-scene effects '{"add":{"layer":"background","effect":"waves","params":{"amplitude":6,"frequency":3}}}'
```

*Editor:* choose the sky layer and, in the *Effects* panel (the tab beside the *Inspector*), *Add an
effect* and *Waves*. The effect becomes a row of that panel: open it and set *Amplitude* to 6 and
*Frequency* to 3. In the command, `amplitude` and `frequency` are the ids
of those parameters in `scene.json`.

Effects are in [effects.md](../effects.md).

### 5. A user property

A user property is something that the person using your wallpaper can change without touching
anything else. This one lets them pick the sun's color:

```
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"color_sun","label":"Sun color"}'
reiwand-ctl scene act my-scene property '{"id":"color_sun","default":"#ffd23f"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/1/fill","value":{"prop":"color_sun"}}'
```

*Editor:* *Options* panel (on the left of the *Wallpaper* layout, Ctrl+6; in *Scene*, *Window* › *Options* opens it), *Add property* of
type *Color*: it is born with the id
`color_1`; with *Rename…* call it `color_sun`, and give it the label “Sun color” and the prefilled
value `#ffd23f`. In the sun layer's *Fill color*, click the diamond and choose *Follow a user
property* and the property. The fill ends up as `{ "prop": "color_sun" }`, with nothing else, and
the sun is shown in the property's color.

The property is declared in `manifest.json` and the layer follows it with
`{ "prop": "color_sun" }`.

### 6. Check

```
reiwand-ctl pack check my-scene
```

*Editor:* the *Problems* panel, below the canvas beside the *Timeline* (its tab counts them, and so
does the bar at the foot of the window), which updates as you work and says *No problems* when
everything is fine. Click *Save* before viewing the scene.

It prints nothing if everything is fine. If something is wrong, it gives the path of the field and
what it expected. This command exits with an error when there are problems, which is why it is
useful in scripts. If the scene has scripts (`scripts.md`), it also compiles them without running
them and reports, with the file and line, a syntax error or a missing script.

### 7. See it

```
reiwand-ctl scene preview my-scene
```

*Editor:* *View as wallpaper*, at the top right of the editor.

It shows the scene on the desktop for 60 seconds without applying it as the wallpaper, and needs
the service to be running. Without it, you can build the package and draw a frame:

```
reiwand-ctl pack build my-scene -o my.wpak
reiwand-ctl scene bench my.wpak --frames 360 --shot frame.png
```

`bench` draws the scene off screen, reports how much each frame costs, and saves the last one as
an image: with `--frames 360` that is the 6 s mark at 60 frames per second, when the sun is halfway.

`bench` does not run scripts. To see a scene with scripts without a desktop, and at a fixed time,
use `reiwand-ctl scene render my-scene -o view.png --time-of-day 15:42` (`cli.md`,
“`scene render`”). A scene with particles looks empty in the first frame if its system has no
`prewarm` (`particles-and-sound.md`).

To really apply it as the wallpaper: `reiwand-ctl scene apply my-scene`, which at the end says what
it showed and where (`applied: My first scene 0.1.0 on DP-1, quality Q3`), and to remove it,
`reiwand-ctl clear` (and `reiwand-ctl restore` brings it back). If you change the scene, apply it
again: you do not need to bump the version, but it takes a while to show while the daemon builds
and loads it.

### And now

The scene you just made is the [`examples/first-scene`](../examples/first-scene) folder; its whole
`scene.json` is:

```json
{
  "schema": "reiwand.scene/2",
  "canvas": {
    "size": [1920, 1080], "fit": "fill", "background": "#000000", "blend_space": "linear",
    "clear": true, "hdr": false
  },
  "clock": { "max_fps": "display", "poster_time": 0.0, "seed": 0, "step": 60 },
  "assets": {
    "sky": {
      "path": "assets/sky.png", "animated": false, "durations": [], "face": null,
      "filter": "linear", "fps": null, "frames": null, "linear": false, "mipmaps": null, "rects": [],
      "shapes": {}, "wrap": "clamp"
    }
  },
  "layers": [
    {
      "blend_space": "canvas", "id": "background", "type": "image", "image": "sky",
      "size": [1920, 1080],
      "effects": [
        {
          "effect": "waves",
          "params": {
            "amplitude": 6, "frequency": 3, "angle2": 90.0, "count": 1.0, "direction": 0.0,
            "perspective": 0.0, "phase": "", "speed": 1.0
          },
          "enabled": true, "optional": false
        }
      ],
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "opacity": 1.0,
      "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    },
    {
      "blend_space": "canvas", "id": "sun", "type": "shape",
      "shape": { "ellipse": { "size": [220, 220] } },
      "fill": { "prop": "color_sun", "invert": false },
      "transform": {
        "position": {
          "timeline": "lap", "keys": [[0, [260, 820], "in_out_sine"], [12, [1660, 260]]],
          "invert": false
        },
        "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0]
      },
      "blend": "normal", "cache": false, "depth_test": false, "effects": [], "opacity": 1.0,
      "parallax": [0, 0], "solid": false, "visible": true
    }
  ],
  "timelines": [
    {
      "id": "lap", "duration": 12, "loop": "pingpong", "autoplay": true, "delay": 0.0,
      "events": [], "fps": null, "frames": null, "speed": 1.0, "wrap": false
    }
  ],
  "effects": {}, "ext": {}, "listens": [], "particles": {}, "pools": {}, "scripts": [],
  "templates": {}
}
```

Continue with [scene.md](../scene.md) for the rest of the file, with [layers.md](../layers.md) for the
layer types, and with [package.md](../package.md) when you want to give your work a name, a license
and a signature and share it.
