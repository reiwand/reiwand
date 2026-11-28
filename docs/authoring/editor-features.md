# Scene editor

The *Scenes* page of the app creates a scene from a starting point (empty, particles, layered
parallax or text clock), opens a scene folder, or edits a copy of a scene from the library, in a new
folder inside the documents folder (`reiwand/<title>`) or in the one you choose. A scene being
edited is a folder with `manifest.json`, `scene.json` and its assets in `assets/`: the editor has no
format of its own and changes the same files the engine reads. New scenes use the second version of
the manifest and are born valid: for all audiences, no flashes, an original work and with the inputs
they already use declared, until you change that data. `reiwand-app --page scenes` opens that page
and, as with the image editor, a request for another page does not close an editor with unsaved
changes.

What the editor can do, by area:

- [The parts of the editor](editor-features/parts.md): layers, the canvas, the inspector and the timeline.
- [The package, properties and assets](editor-features/package-and-assets.md): the package, properties and presets, and assets.
- [Problems and painted masks](editor-features/problems-and-masks.md): the problems list, messages and masks painted per channel.
- [Meshes, clips and chains](editor-features/meshes.md): meshes from pictures, bones, clips, chains, physics and the pointer.
- [Particle tools](editor-features/particles.md): particle tools on the preview, the system inspector, colliders and forces.
- [Layers from files and 3D models](editor-features/files-and-3d.md): layers from files and importing 3D models.
- [Converting FBX and OBJ models to glTF with Blender](editor-features/fbx-and-obj.md): turning other model formats into `.glb`.
