# User properties

A user property is a setting that the user changes without touching your scene: a color, a
slider, a choice, an image of their own. You declare the properties in `manifest.json`, under
`properties`, and the scene uses them by their id. The program shows them as a form, in
*Scene settings*, and remembers what the user chooses.

All the examples on these pages come from
[`examples/properties`](examples/properties/), and they pass `reiwand-ctl pack check`.

User properties, by topic:

- [Declaring properties](properties/declaring.md): declaring and using a property, the common fields and the types.
- [Conditions, presets and translations](properties/behavior.md): conditions, icons, values by quality, presets, translations and what the user sees.

## Not covered here

- `scripts`: reading and moving properties from code: [scripts.md](scripts.md).
- The whole manifest and the permissions: [package.md](package.md).
