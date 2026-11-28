# What each example shows

These pages list every feature of the scene format (`docs/specs/scene.md`) and the example or
gallery scene that shows it, so that you can see each one working. `scripts/coverage.py` works
the list out from the format itself and checks that each example named here really uses its
feature; `scripts/docs-examples.sh` runs it, and fails when the format gains a feature that no
example shows. `scripts/coverage-pending.txt` lists the features that have no example yet.

Names such as `Render.trail` or `EmitShape:circle` are the types and fields of the frozen schema of
`scene.json` (`crates/reiwand-fill/frozen/scene.json`): `Type.field` is a field written with a value
other than its default, `Type:variant` one form of a type that has several, and `Type=value` one
value of a list of choices. `Property` names come from the schema of the user properties in
`manifest.json`.

What the examples cover, by area:

- [Features, effects, blends and layers](coverage/features.md): `requires.features`, effects, blend modes and layers.
- [Values, triggers, files and scripts](coverage/values-and-scripts.md): values, triggers, file types, properties and script interfaces.
- [Everything else in `scene.json`](coverage/scene.md): the rest of the scene file.
