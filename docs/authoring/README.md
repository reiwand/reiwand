# Creating scenes

A **scene** is an animated wallpaper that you draw with layers: images, shapes, text, particles,
lights, 3D models, sound. It moves with time, with the sound that is playing, with the pointer, or
with whatever the user picks in their settings. This guide teaches you to make one from scratch,
without knowing anything about the program.

A scene is a folder with three things:

```
my-scene/
  manifest.json   the name, the author, the license and the user properties
  scene.json      the layers and everything that moves
  assets/         the images, sounds, fonts and models
```

The two files are text (JSON) and you can write them by hand. There are three ways to make the
same scene, and you can mix them: all three change the same files.

- **By hand**, with a text editor. `reiwand-ctl pack check` tells you what is wrong and where.
- **With the app's editor**: the *Scenes* page of `reiwand-app`. It is the most comfortable way to
  place layers with the mouse and move values on a timeline.
- **With the terminal**: the `reiwand-ctl scene ...` commands. They let a script do the same
  things the editor does.

Start here:

- [Your first scene](start/first-scene.md): from an empty folder to a scene on your desktop, step by step.

## What you need

- The program installed: the app (`reiwand-app`), the service that draws the wallpapers, and the
  `reiwand-ctl` command.
- An image of your own, PNG, JPEG or WebP, for the tutorial (here it is called `sky.png`).
- A text editor, if you are going to write by hand.

Most `reiwand-ctl scene` commands do not need the service to be running: they create, change,
check and measure the scene. Only the preview on the desktop needs it.

## The pages of this guide

| Page | What it covers |
|------|----------------|
| [README.md](README.md) | This page: what a scene is and your first scene. |
| [scene.md](scene.md) | The whole file: canvas, clock, assets, cost and quality, values that move, inputs. |
| [layers.md](layers.md) | Each layer type, with an example and a table of fields. |
| [effects.md](effects.md) | The effects the program ships with, masks, and how to write your own effect. |
| [particles-and-sound.md](particles-and-sound.md) | Particle systems, sound, and reacting to sound. |
| [properties.md](properties.md) | User properties, presets and translations. |
| [3d.md](3d.md) | Lights, 3D models, shadows, camera and depth. |
| [meshes.md](meshes.md) | Images bent by a skeleton, clips, physics and inverse kinematics. |
| [editor.md](editor.md) | The app's editor, tool by tool. |
| [actions.md](actions.md) | The editor's actions, which you call with `reiwand-ctl scene act`. |
| [scripts.md](scripts.md) | JavaScript code for what cannot be described with values. |
| [package.md](package.md) | The manifest, signing, packaging and publishing. |
| [transitions.md](transitions.md) | Making and testing a wallpaper's own transition, and its limits. |
| [cli.md](cli.md) | The `reiwand-ctl` commands to create, measure and publish. |
| [provider-plugins.md](provider-plugins.md) | Not a scene: a TypeScript plugin that adds a new source of wallpapers. |
| [coverage.md](coverage.md) | Which example shows each feature of the format. |

All the complete examples are in the [`examples/`](examples/) folder, one scene folder per
example. Each one is checked with `reiwand-ctl pack check`, and you can open it in the editor or
copy it and change it. The one exception is [`provider-plugin`](examples/provider-plugin/), a
provider plugin, which a test of `crates/reiwand-plugin` installs and runs.

| Example | What it teaches |
|---------|-----------------|
| [`first-scene`](examples/first-scene/) | The tutorial below. |
| [`canvas-and-clock`](examples/canvas-and-clock/) | Canvas, parallax, glow, camera, clock, time and date. |
| [`shapes`](examples/shapes/), [`text`](examples/text/), [`video`](examples/video/), [`groups-and-masks`](examples/groups-and-masks/) | The layers. |
| [`animation-and-interaction`](examples/animation-and-interaction/) | Sprite sheets, clicks, and sound that plays on an event. |
| [`values`](examples/values/) | Values that follow the sound, the pointer, the clock and keys. |
| [`stacked-effects`](examples/stacked-effects/), [`own-effect`](examples/own-effect/), [`own-transition`](examples/own-transition/) | Effects, masks, your own effects and your own transition. |
| [`effects-distort`](examples/effects-distort/) | A portal in a nebula: `twirl`, `spin`, `displace`, `blur_radial`, `sway`, `perspective`, `transform` and a `lens_distort` masked to a circle. |
| [`effects-reveal`](examples/effects-reveal/) | A valley from day to night: `blend_gradient`, `blend_texture`, `reveal`, `iris`, `dissolve`, and an effect masked by another layer. |
| [`effects-color`](examples/effects-color/) | One harbour in five grades (`color_lut` with a `.cube` table, `tint`, `posterize`, `local_contrast`, `edges`), a `chroma_key` on a green-screen video and a layer mixed in sRGB. |
| [`effects-light`](examples/effects-light/) | A relic with `shine`, `glitter`, `pulse`, `electric` and `trail`, and a glow measured in screen pixels. |
| [`blend-modes`](examples/blend-modes/) | All 34 blend modes side by side. |
| [`timelines`](examples/timelines/) | Timelines started at the start, on the beat, by a click, by the pointer entering or leaving a layer, by an event, by a button in the settings and by the media player; delay, backwards, frames at their own rate and `wrap`. |
| [`values-extras`](examples/values-extras/) | Absolute levels per channel, surges, the beat, the pointer's speed and presence, `invert`, Bézier curves and handles, color parts, the day, minute and second, a layer fed into an effect, and the song that plays. |
| [`canvas-extras`](examples/canvas-extras/) | A canvas that is never cleared, HDR with a tone map, a camera that rolls and trembles, a parallax that drifts and follows the tilt of the phone, wind and the clock step. |
| [`particles`](examples/particles/), [`advanced-particles`](examples/advanced-particles/), [`propeller`](examples/propeller/), [`sound`](examples/sound/) | Particles and sound. |
| [`properties`](examples/properties/) | Every type of user property, presets and translations. |
| [`lights`](examples/lights/), [`model-3d`](examples/model-3d/), [`depth`](examples/depth/), [`mesh-with-bones`](examples/mesh-with-bones/) | Lights, 3D, depth and bones. |

## Three things worth knowing from the start

1. **Colors** are written `"#rrggbb"` or `"#rrggbbaa"`, always with 6 or 8 digits.
2. **A value can move.** Almost any number, color, text or switch can be a fixed value or an
   object with a source: a property (`{ "prop": ... }`), keyframes (`{ "keys": ... }`), the sound
   (`{ "audio": ... }`), the pointer (`{ "cursor": ... }`) or the clock (`{ "clock": ... }`). It is
   in [scene.md](scene.md).
3. **Scenes run on phones.** Every frame costs work; [scene.md](scene.md) explains how to estimate
   it and bring it down.
