# Porting a shader to WGSL

Package shaders are written in WGSL and nothing else: the engine never compiles GLSL or HLSL. An
effect of another program becomes one package effect: its passes, render targets and copies become
`passes`, `targets`, `copy` and `swap`, its compile-time switches `options`, its uniforms `params`,
and its fragment and vertex programs entry points of one `.wgsl` file. The rules of thumb:

- Write one `@fragment` function per pass, `fn fs_<name>(in: VertexOut) -> @location(0)
  vec4<f32>`, and return the color instead of writing `gl_FragColor` or an `out` variable. Colors
  are premultiplied by alpha; `straight(c)` undoes it. Where alpha is 0 no color is kept:
  `straight` gives black there, also after an effect that only lowered the alpha. An original that
  keeps the color under a pixel it made transparent, and later gives that pixel alpha again (rays,
  glow or blur spreading over it), takes the color from the `source` input, `straight` of the
  layer before its effects, and the alpha from its own input. When the layers below show the same
  picture, setting the layer's `mask` to the same picture also works: it applies after the
  effects and hides what spread past it.
- Coordinates go from (0, 0) at the top left to (1, 1) at the bottom right, in `in.uv`. Flip `y`
  where the original counts from the bottom (`1.0 - uv.y`).
- Move what the original computes per vertex but is affine (offsets, scales, rotations of the
  coordinates) into the fragment entry point; keep a `vertex` entry with a `grid` only to bend the
  layer itself.
- Turn every `#ifdef`, `#if` or combo into an `override` with `options`, or into an `if` on a
  parameter when it changes at run time, and every texture slot into an `inputs` entry and a
  `texture` parameter.
- Give every loop a literal limit: `for (var i = 0u; i < min(count, 32u); i++)`.

| HLSL or GLSL | WGSL |
|--------------|------|
| `float`, `float2`, `vec3`, `half4`, `int2`, `bool` | `f32`, `vec2<f32>`, `vec3<f32>`, `vec4<f32>`, `vec2<i32>`, `bool` |
| `float2x2`, `mat3` | `mat2x2<f32>`, `mat3x3<f32>` |
| `mul(m, v)` in HLSL, `m * v` in GLSL | `v * m` for an HLSL matrix written by rows, `m * v` for a GLSL one |
| `frac(x)` | `fract(x)` |
| `saturate(x)` | `clamp(x, 0.0, 1.0)` |
| `lerp(a, b, t)` | `mix(a, b, t)` |
| `fmod(a, b)` (HLSL, truncated) | `a - b * trunc(a / b)` |
| `mod(a, b)` (GLSL, floored) | `a - b * floor(a / b)` |
| `atan2(y, x)`, GLSL `atan(y, x)` | `atan2(y, x)` |
| `rsqrt`, `inversesqrt` | `inverseSqrt` |
| `ddx`, `ddy`, `dFdx`, `dFdy` | `dpdx`, `dpdy` |
| `tex2D(s, uv)`, `texture(s, uv)`, `texture2D` | `sample(aux0, uv)`, or `tap(uv)` for the layer itself |
| `tex2Dlod`, `textureLod` | `textureSampleLevel(t, samp, uv, level)` |
| `discard` / `clip(x)` | `discard` / `if x < 0.0 { discard; }` |
| `static const`, `#define N 8` | `const N: u32 = 8u;` |
| a uniform shown as a slider, color or point | a `float`, `color` or `position` parameter read with `param(n)` |
| a combo or `#define` chosen before compiling | an `override` named by `options` |
| the time and the time since the last frame | `frame.time`, `frame.dt` |
| the time of day from 0 to 1 | `daytime()` |
| the pointer and its previous position | `frame.cursor`, `cursor_last()`, or `cursor_uv()` on the layer |
| the size of the screen and of a texture | `frame.resolution`, `tex_size(i)` and `fx.texel` |
| 16, 32 or 64 bands of sound, per channel | `band_of(i, 16u)`, `band_left_of(i, 16u)`, `band_right_of(i, 32u)`, `band_left(i)`, `band_right(i)` |
| the parallax shift | `parallax()` |
| the layer's color and opacity | `layer_color()`, `layer_alpha()` |
| greyscale, HSV, 2D rotation and blending helpers | `greyscale`, `hsv`, `from_hsv`, `rotate2`, `blend_mode` |

Package shaders declare no bindings of their own: the input, the mask and up to eight more textures
come through `inputs` and `params`. `reiwand-ctl scene act DIR import_effect` checks the result the same way a
package reader does and says what to change.
