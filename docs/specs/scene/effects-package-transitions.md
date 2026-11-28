# Package transitions

A scene may bring its own transition, for when it comes in and, if it wants, for when it leaves.
It is declared in `scene.json` under `transition`, with up to two passages, `in` and `out`; each
passage names its own two files, a descriptor and a shader, like a package effect:

```json
"transition": {
  "in": {
    "descriptor": "transitions/porthole.json",
    "wgsl": "transitions/porthole.wgsl",
    "duration_ms": 1200,
    "curve": "in_out_sine",
    "params": { "soft": 0.2 }
  }
}
```

```json
{
  "id": "porthole",
  "stage": "transition",
  "cost": { "taps": 1, "passes": 1, "scale": 1 },
  "params": [{ "id": "soft", "type": "float", "prefill": 0.1, "min": 0, "max": 1 }]
}
```

```wgsl
fn transition(c: Crossing) -> Crossing {
    var o = c;
    let soft = param(0u).x;
    let d = length((c.uv - 0.5) * vec2<f32>(c.resolution.x / c.resolution.y, 1.0)) / 0.75;
    let k = smoothstep(c.progress * (1.0 + soft) - soft, c.progress * (1.0 + soft), d);
    o.color = mix(after(c.uv), before(c.uv), k);
    return o;
}
```

The shader defines `fn transition(c: Crossing) -> Crossing`, called once per pixel of each screen
for every frame of the transition. `Crossing` carries `uv` (0 to 1, top left origin), `resolution`
in pixels, `progress` (0 to 1, already shaped by the curve), `quality` (0 to 3, the level the screen
draws at) and `color`, which arrives as the plain fade of the two pictures. `before(uv)` samples the
picture that was showing and `after(uv)` the one that comes in; `param(i)` reads the parameters in
the order of the descriptor, and the shared helpers of package effects (`TAU`, `LUMA`, the Oklab,
noise and dither functions) are there too. The color the shader leaves is drawn opaque. For an
`out` passage, `before` is this scene and `after` is a plain picture in the background color of the
wallpaper that comes next.

`duration_ms` goes from 100 to 10000 (prefilled 1000); `curve` takes the easing names of
keyframes (`in_out_sine`, a cubic Bézier…) (prefilled `linear`) and its result is kept between 0 and 1; `params` holds
literal values for at most 24 parameters, each one named in the descriptor and of its type. The
descriptor has the `transition` stage and goes through every check of package effects: the
sandboxed shader, the declared cost compared with the measured one, the strikes. A transition takes
no picture, lut, mask or emitter parameter, and it may cost at most 8 units, so it is never
heavier than a medium effect; it plays at the quality levels where an optional effect of its class
plays (a light one everywhere, a medium one only at levels 2 and 3).

A transition is not part of the picture: one that fails its checks does not refuse the scene. The
check of the package (`reiwand-ctl pack check`) lists the reason in words, the log notes it, and the
screen uses the plain fade of the settings instead. The transition chosen in the settings still
rules over it: `transition.kind` (or `transition.kind.<output>` for one screen) is `own` when the user chooses nothing else, which plays the wallpaper's transition; `fade` always gives the plain fade and `none` a
cut. `transition.less_motion` (`system`, `always`, `never`) turns every transition into
the plain fade while it reduces, and `none` stays a cut. The state of each screen says which one
played at its last change and why (`own`, `chosen`, `reduced`, `fallen`, `plain`).
When a wallpaper with an `out` passage gives way to one with an `in` passage, `out` plays first
and `in` after it, each with its own duration; the work exists only while a transition plays and
each screen plays its own. The shaders of the passage are built away from the drawing while the
new wallpaper loads, within the same build limit as package effects (the log says
when it is passed); the passage starts once they are ready, so it always plays from its first
frame, and the old wallpaper stays on screen meanwhile. A build that fails or passes the limit
gives the plain fade.

The phone plays package transitions with the same engine (feature `transition` is `yes` there).
Its choice is `transition.kind` for the screen (`own`, `fade`, `none`). Less motion on the phone
(the system's «remove animations», or `transition.less_motion` set to `always`) turns every change
into a cut, not a fade. With battery saver or a low battery away from the charger the wallpaper
stands still and every change is a cut; below the battery level where the lowest quality is used
the wallpaper's transition gives way to the plain fade. A passage that does
not play at the quality the screen draws at is left out and the other one, if any, still plays; a
transition that fails its checks, does not build in time or fails to draw gives the plain fade,
logged and never shown to the person. A change of wallpaper while a passage plays drops the
passage and the plain fade carries on to the last choice. The two wallpapers and the passage's
textures are released the moment the change ends.
