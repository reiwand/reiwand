# Pointer areas and sound bindings and Shared parts

On this page: pointer areas and sound bindings, shared parts.

## Pointer areas and sound bindings

*Pointer* in the inspector of a group, image, video, text, shape, solid, spectrum or mesh layer
has three states: lets the pointer through, takes it over its whole box, or takes it where the
layer is opaque, with a slider for the opacity (`layer` action `touch`; the same on the command line
with `reiwand-ctl scene act DIR layer '{"touch": {"layer": ID, "takes": "alpha", "alpha": 0.3}}'`).
While a layer that takes the pointer is selected, the canvas outlines the area of every layer that
takes it (solid line for the whole box, dashed for the opacity threshold, with its percentage), the
selected one stronger. The areas come from the editor query `{"touch": true}` (`rigs::query`), which
answers `{layers: [{id, pointer, takes, alpha, bones}]}` for the layers with `solid` or `hit` bones.
The bones that take the pointer of a mesh or model are chosen from the bones of its file in the rig
panel.

The binding menu of a value offers *Follow the sound* with the five named levels and *A band or a
range of bands…*, a picker with the 16, 32 or 64 bands drawn as a strip (click a band or drag over
several, with their pitch in Hz or kHz), how several bands count (average or loudest) and the channel
(both, left or right). *Follow the pointer* picks `x`, `y`, `inside` or `speed`. Both go through the
editor query `{"bands": {...}}`: without `pick` or `direction` it answers `{of, bands: [{index, from,
to}], named: {bass, mid, treble}, levels, pointer}` for `of` 16, 32 or 64; with `pick` (a level name,
a band number or `[first, last]`, with optional `of`, `reduce` and `channel`) or `direction`, and the
`base` source being replaced, it answers `{source, label, hertz}` where `source` keeps the shaping
(`scale`, `offset`, `clamp`, `bounds`, ...) of `base` and drops what only belongs to the old source,
or `{error}`. The value is then written with `set`, one undoable step.

## Shared parts

The *Assets* panel of the scene editor has two tools for `assets` packages (`package.md`, "Shared
resources"). *Make an assets package* lists the layers, particle systems, scripts and package effects
of the scene with the selected layer checked; each checked part takes a title and a thumbnail, and the
package takes a title, an id, a license, the name its parts are credited to and a preview picture.
It saves the scene, writes a new folder with `assets.json`, the files the parts use, the credits of
the scene that cover them and the pictures made into jpegs of at most 1024 pixels and 1 MiB, and
offers to open that folder in the editor, where it is built like a scene. `reiwand-ctl pack new-assets
SCENE OUT --part KIND:ID… --id ID --title TITLE --license SPDX --attribution NAME [--thumbnail FILE]
[--part-thumbnail KIND:ID=FILE] [--part-title KIND:ID=TITLE]` does the same.

*Import from an assets package* opens a `.wpak`, shows each part with its thumbnail and, before
anything is copied, the license and credit of every file it brings; the checked parts are imported
with `import_assets`, one step of the history. `reiwand-ctl pack parts FILE [--json]` lists the same.
