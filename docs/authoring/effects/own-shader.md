# Your own effect: the shader

On this page: the shader, trying it, other shaders and own transitions.

## The shader

A fused effect (`fuse`) defines a function whose name carries the id of the effect:

- with `"fuse": "color"`: `fn color_<id>(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32>`, on colors
  premultiplied by alpha;
- with `"fuse": "uv"`: `fn uv_<id>(uv: vec2<f32>) -> vec2<f32>`, which says where each point reads
  from.

```wgsl
fn color_duotone(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let c = straight(color);
    let luma = dot(c.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let toned = mix(from_linear(param(0u).rgb), from_linear(param(1u).rgb), luma);
    let rgb = mix(c.rgb, toned, param(2u).x * mask_at(uv));
    return vec4<f32>(rgb * c.a, c.a);
}
```

An effect with passes defines one function per pass:

```wgsl
@fragment
fn fs_haze(in: VertexOut) -> @location(0) vec4<f32> {
    let c = tap(in.uv);
    let haze = from_linear(param(0u).rgb) * c.a;
    let amount = param(1u).x * mask_at(in.uv) * in.uv.y;
    return vec4<f32>(mix(c.rgb, haze, amount), c.a);
}
```

```json
{
  "id": "haze", "name": { "en": "Haze" }, "mask_mode": "strength",
  "params": [
    {
      "id": "color", "type": "color", "channels": null, "help": {}, "label": {}, "max": null,
      "min": null, "paint": null, "prefill": "#c8d4f0", "tool": null, "tools": [], "unit": null,
      "uses": null, "values": [], "waits_for": null, "yields_to": null
    },
    {
      "id": "amount", "type": "float", "min": 0, "max": 1, "channels": null, "help": {},
      "label": {}, "paint": null, "prefill": 0.4, "tool": null, "tools": [], "unit": null,
      "uses": null, "values": [], "waits_for": null, "yields_to": null
    }
  ],
  "passes": [
    {
      "entry": "fs_haze", "blend": null, "copy": null, "grid": null, "inputs": ["previous"],
      "output": "next", "scale": 1, "swap": null, "target": null, "vertex": null
    }
  ],
  "cost": { "taps": 1, "passes": 1, "scale": 1 }, "audio": [], "bounded": [], "category": null,
  "compose": false, "description": null, "fuse": null, "lighter": null, "min_fps": null,
  "options": [], "pace": null, "quality": {}, "reach": null, "solve": null, "stage": "layer",
  "state": null, "targets": {}, "uses": [], "velocity": null
}
```

Coordinates go from (0, 0) at the top left to (1, 1) at the bottom right. What the shader can use:

| Name | What it is |
|------|------------|
| `param(n)` | Parameter number *n* (from `0u`), in `.x`, `.xy` or `.rgb`. |
| `tap(uv)`, `sample(t, uv)` | Read the input image, or another texture. |
| `mask_at(uv)` | The mask of the use, from 0 to 1. |
| `px(pixels)` | Pixels converted to coordinate space. |
| `straight(c)` | Removes the alpha premultiplication. |
| `to_linear`, `from_linear` | Convert between the effect's values and linear light according to `canvas.blend_space`; effects do not read the `blend_space` of the layer. |
| `frame.time`, `frame.dt` | The time and what has passed since the last frame. |
| `frame.cursor`, `frame.inside`, `frame.buttons` | The pointer. |
| `frame.bass`, `mid`, `treble`, `level`, `beat`, `band(i)` | The sound. |
| `layer_uv(uv)`, `layer_extent()` | Where the point falls inside the layer, and its size. |
| `oklab`, `from_oklab`, `gauss`, `hash`, `hash2`, `noise1`, `value_noise`, `gradient_noise`, `fbm(p, octaves)`, `bayer(pixel)` | Color and noise helpers. |

An effect cannot declare binding variables of its own: the input image, the mask and up to eight
more images arrive through `inputs` and `params`.

Shader rules: each loop has to count with an integer variable from a literal to a literal limit or
a constant (`for (var i = 0u; i < 16u; i++)`), and at most 1,024 iterations; `while` is not
allowed. The work it can do is checked before it reaches the graphics driver. If something fails,
the program says what: `/effects/duotone: ...` with the line and column of the error.

## Trying it

```
reiwand-ctl scene act MY-FOLDER import_effect '{"descriptor":"effect.json"}'
reiwand-ctl pack check MY-FOLDER
```

`import-effect` copies the descriptor and the `.wgsl` next to it into the folder and checks them
the way whoever opens a package does. With `reiwand-ctl pack build` and `scene bench` you measure
its cost. In the editor, *New effect* starts from a template and shows the errors in the code.

## Other shaders

With `"stage": "material"`, the shader defines `fn material(s: Surface) -> Surface` and shapes the
material of a model (see [3d.md](../3d.md)); with `"stage": "particle"`, it defines
`fn particle(p: Sprite) -> Sprite` and changes how each particle is drawn (see
[particles-and-sound.md](../particles-and-sound.md)).

A particle shader can also define `fn pixel(p: Pixel) -> vec4<f32>`, which runs for each pixel of
the sprite before the alpha. `p.sample` is the texture sample, `p.color` the color and alpha of the
particle (without the brightness and not multiplied by alpha), `p.brightness` the brightness of the
system, `p.alpha` the alpha (`sample.a * color.a`), `p.uv`, `p.age` and `p.life`. It returns
straight color and alpha and the engine multiplies afterwards. So that the core reaches white
without lightening the translucent edge:

```wgsl
fn pixel(p: Pixel) -> vec4<f32> {
    let lit = min(p.sample.rgb * p.color.rgb * p.brightness, vec3<f32>(1.0));
    return vec4<f32>(lit, p.sample.a * p.color.a);
}
```

With `pixel`, the brightness is no longer multiplied into the sprite color (the function applies
it). Without it everything looks as always. Trails and ropes do not call it; `refract` only reads
its alpha.

## Your own transition

A wallpaper can bring its own transition for when it comes in and, if it wants, another for when it
leaves. It is declared in `scene.json`, under `transition`, with `in` and `out`; each one names its
descriptor (`"stage": "transition"`) and its `.wgsl`, lasts from 100 to 10000 ms (`duration_ms`),
carries a curve (`curve`, the same names as the keys) and fixed values for its parameters
(`params`). The shader defines `fn transition(c: Crossing) -> Crossing`: `before(uv)` is the image
that was being seen, `after(uv)` the one coming in and `c.progress` goes from 0 to 1; the color it
leaves is drawn opaque. See [`examples/own-transition`](../examples/own-transition/scene.json).

A transition cannot use images, color tables, masks or emitters, and it costs at most 8 units: a
light one is seen at all quality levels and a medium one only at levels 2 and 3. If it does not
pass the checks, the scene still opens: `reiwand-ctl pack check` says why and the wallpaper comes
in with the usual fade. What the user chooses rules: without a transition, the change is direct,
and with reduced motion it is a fade. When going from a wallpaper with `out` to one with `in`, the
exit is seen first and then the entrance. On the phone it looks the same, except that with reduced
motion the change is direct, and while it saves battery or the chosen quality does not allow it, it
comes in with the fade.
