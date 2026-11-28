# Image and video layers

On this page: `image` and `video`.

## Image layer (`image`)

Draws an image asset. Without `size`, the image is shown at its real size, centered; with `size`
it is stretched to that size.

```json
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
    }
```

| Field | Prefill | What it does |
|-------|---------|--------------|
| `image` | required | The id of the asset, or a value: `{ "prop": "picture" }` for a file property or `{ "prop": "style", "map": { "night": "sky" } }` to choose among assets. |
| `size` | the size of the image | `[width, height]` in pixels, from 0.001 to 100000. |
| `color` | white | Color that multiplies the image. |
| `playback` | | For sprite sheets, GIFs and videos. Below. |

If the asset has `wrap` `repeat` or `mirror` and `size` is larger than the image, the image repeats
instead of stretching. An image with a file property shows what the user chooses, and any new file
keeps the clock, the timelines and the particles of the scene.

`playback` rules moving images and videos:

| Field | Prefill | Range | What it does |
|-------|---------|-------|--------------|
| `rate` | 1 | 0 to 16 (video: 0, or 0.1 to 4) | The speed. With 0 a video stops. |
| `playing` | `true` | | Play or stop. |
| `loop` | `true` | | Repeat at the end. |
| `start`, `end` | 0, the end | seconds | The stretch that plays. |
| `frame` | | | Which frame of a sheet is shown (can follow keys, the pointer or the sound). |
| `own_clock` | `false` | | Only on video layers: its own clock, with its own rate, stretch and pause. It costs one more decoder. |

A sprite sheet (an asset with `atlas` or `rects`, see [scene.md](../scene.md)) plays its frames with
the scene clock: all the layers that show the same sheet at the same rate show the same frame. At
the lowest quality level it stays on the first frame of the stretch.

```json
    {
      "blend_space": "canvas", "id": "dance", "type": "image", "image": "ball", "size": [256, 256],
      "transform": {
        "position": [700, 540], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "playback": {
        "rate": 1, "loop": true, "end": "end", "own_clock": false, "playing": true, "start": 0.0
      },
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false, "visible": true
    }
```

## Video layer (`video`)

Plays a video asset (WebM, MP4 or MKV), looping unless you say otherwise.

```json
    {
      "blend_space": "canvas", "id": "film", "type": "video", "video": "frames",
      "size": [1920, 960],
      "playback": {
        "rate": 1, "loop": true, "start": 0, "end": "end", "own_clock": false, "playing": true
      },
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false,
      "transform": { "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0], "skew": [0.0, 0.0] },
      "visible": true
    }
```

It has the same fields as an image layer, but with `video` instead of `image` (the id of the asset,
or a value with `prop` and `map`). All the layers that show the same video share a decoder and a
clock, which the first of them drives; `own_clock` gives it its own clock:

```json
    {
      "blend_space": "canvas", "id": "small", "type": "video", "video": "frames",
      "size": [384, 192],
      "transform": {
        "position": [1700, 960], "origin": "anchor", "rotation": 0.0, "scale": [1.0, 1.0],
        "skew": [0.0, 0.0]
      },
      "playback": {
        "own_clock": true, "rate": 0.5, "end": "end", "loop": true, "playing": true, "start": 0.0
      },
      "blend": "normal", "cache": false, "color": "#ffffff", "depth_test": false, "effects": [],
      "opacity": 1.0, "parallax": [0, 0], "solid": false, "visible": true
    }
```

While no visible layer shows the video, it stands still and asks for no frames. On battery its
decoder does not go faster than `clock.max_fps`. With a file property that accepts `video` the user
can choose their own video.
