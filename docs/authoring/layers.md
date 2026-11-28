# Layers

A scene is drawn with layers, from bottom to top: the first in the list stays at the back and the
last on top. This page says what types of layer there are, what fields each one has and how they
are placed. The effects of a layer are in [effects.md](effects.md), the values that move in
[scene.md](scene.md).

All the examples come from the folders of [`examples/`](examples/) and pass
`reiwand-ctl pack check`.

The layers, by topic:

- [Layer basics](layers/basics.md): placing, blending, masks, following and reacting.
- [Image and video layers](layers/image-and-video.md): `image` and `video`.
- [Text layer (`text`)](layers/text.md): its fonts, lines and effects on letters.
- [Shape, solid and spectrum layers](layers/shapes.md): `shape`, `solid` and `spectrum`.
- [Group and adjustment layers](layers/groups.md): `group` and `adjustment`.

## Layers explained on another page

- `particles`: a particle system. See [particles-and-sound.md](particles-and-sound.md).
- `sound`: a background sound, without drawing. See [particles-and-sound.md](particles-and-sound.md).
- `light`: a light that illuminates the layers with `lighting`. See [3d.md](3d.md).
- `model`: a 3D model. See [3d.md](3d.md).
- `mesh`: an image bent by a skeleton. See [meshes.md](meshes.md).

## Not covered here

- `lighting` and lights: [3d.md](3d.md).
- `attach`, bones and clips of meshes and models: [meshes.md](meshes.md).
- Layers and templates created by scripts (`templates`, `pools`): [scripts.md](scripts.md).
