# The scene file

This page describes the whole of `scene.json`: the canvas, the clock, the assets, what a scene
costs, the values that move and the inputs. Layers are in [layers.md](layers.md), effects in
[effects.md](effects.md) and user properties in [properties.md](properties.md).

All the examples on these pages come from the folders of [`examples/`](examples/), and each one
passes `reiwand-ctl pack check`.

The scene, part by part:

- [The folder, the clock and the assets](scene/folder.md): what a scene folder holds, the clock of the scene and the files it uses.
- [The canvas](scene/canvas.md): its size and background, parallax, the camera and glow.
- [What a scene costs](scene/cost.md): quality levels, phones, fewer flashes and less motion.
- [Values that move](scene/values.md): properties, keyframes, sound, the pointer, the clock, media and tweaks.
- [Timelines and inputs](scene/timelines-and-inputs.md): timelines, inputs and the sound of the whole scene.

## Not covered here

- Layers and their fields: [layers.md](layers.md).
- Effects and own effects: [effects.md](effects.md).
- Particles and sound: [particles-and-sound.md](particles-and-sound.md).
- User properties and presets: [properties.md](properties.md).
- 3D camera, lights and shadows: [3d.md](3d.md). Meshes with bones: [meshes.md](meshes.md).
- `scripts`, `templates`, `pools` and `ext`: [scripts.md](scripts.md).
- The manifest, signing and the package: [package.md](package.md).
