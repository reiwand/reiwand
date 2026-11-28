# Actions

Actions turn into operations against the current document. The app sends them as
`{"<action>": {<arguments>}}`; `reiwand-ctl scene <command> <folder> …` takes the same arguments,
saves at once and prints the ids of what it added. `reiwand-ctl scene show DIR [POINTER]` prints the
document or the value at a pointer as JSON, and `reiwand-ctl scene history DIR` lists the steps that
undo and redo walk through, the undone ones marked.

The actions, by area:

- [Document and layers](actions/document-and-layers.md): document and layers.
- [Keys, imports and effects](actions/keys-imports-and-effects.md): keys, imports and effects.
- [Bones, clips and expressions](actions/bones-clips-and-expressions.md): bones, clips and expressions.
- [Kinematics, physics and parts](actions/kinematics-physics-and-parts.md): kinematics, physics and parts.
- [Particles](actions/particles.md): particles.
- [Lights and maps](actions/lights-and-maps.md): lights and maps.
- [Scripts, templates and assets](actions/scripts-templates-and-assets.md): scripts, templates and assets.
- [Translations, captures and properties](actions/translations-captures-and-properties.md): translations, captures and properties.
- [Animation and 3D](actions/animation-and-3d.md): animation and 3d.
- [Effects and layer fields](actions/effects-and-layer-fields.md): effects and layer fields.
- [Manifest, canvas, timelines and media](actions/manifest-canvas-timelines-and-media.md): manifest, canvas, timelines and media.
- [Media, assets, ids, keys and timelines](actions/media-assets-and-sprites.md): media, assets and sprites, new ids, keys and timelines.
