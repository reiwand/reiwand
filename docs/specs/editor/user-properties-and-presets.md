# User properties and presets

*User properties* in the *Package* tab lists the properties in the order of the settings form
(`reiwand_scene::order`, by `order` and then id, each group followed by its members) and then the
presets, in one list that scrolls. *Add property* offers every kind (`add_property`) and sends no
label, so a new property shows its id until the author gives it one; the arrows move a property
among the ones of its group (`move_property`) and its menu renames it (`rename_property`), moves it
into a group, last, or out of it, just after its group, and removes it (`remove_property`). Renaming
or removing the property that the inspector shows moves the inspector to the new id, or to the
package details. The rename dialog says that scripts are not rewritten and names the scripts that
name the property in quotes; removing such a property asks first. A property shows the fields of the
JSON Schema of properties (`reiwand-ctl scene schema --property`) for its kind, label, help, group,
order, icon and conditions first, guided by more annotations:

| Annotation | Meaning |
|------------|---------|
| `x-ref` | `group`: the groups of the manifest, named by their labels, and none for a group, since groups do not nest; `icon`: the icons of `scene.md`; `option`: the values of the options of the combo, named by their labels; `media`: the image and video assets |
| `x-condition` | `visible_if` and `enabled_if`, edited on one line |
| `x-color` | a color picker, with transparency only when `alpha` is set |
| `x-scalar` | a value typed as a number, `true`, `false` or a text |

Labels written as `"@key"` show the text of `strings` in the language of the desktop, else in the
default language, as the settings form does. The default of a slider is a slider from `min` to `max`
with its unit, in whole numbers when `integer` is set; `decimals` go from 0 to 6, `max_len` from 1
to 1024, `every` from 10 to 86400 seconds and `accept` offers `image` and `video` as check boxes.
*Add* under the options of a combo appends an option with an empty label and a value no other option
has. A condition is a JSON object, a map of values, or an expression of the language of `scene.md`;
every keystroke checks it as the validator does, so unknown ids, properties without a value,
comparisons of values of other kinds, conditions that are not true or false and conditions that
depend on themselves through other properties show under the field, and only a condition that
checks is recorded, with `condition`. A problem of a group or a condition points at that field of
the property, and a problem of a preset at its id or at the value it sets.

A slider, a combo or a flag shows *Values by quality level* in place of raw JSON: one row for Q0,
Q1, Q2 and Q3 with the widget of the property itself (the slider with its range and unit, the box
with the choices, the switch), *Default* taking a level out. Each edit is the plain `set` or `remove`
of `/manifest/properties/<id>/quality/<q0..q3>`, so `reiwand-ctl scene act DIR set` does the same; the
other kinds of property do not change with the quality level and show no rows. Under the rows the
inspector lists what each level ends up using (the `levels` query of `rig`, `{"levels": {"property":
"<id>"}}`, which applies the same rule as the runtime: a value set for a level holds for the lower
levels until one of their own, and on a slider it is a ceiling that never raises the value the user
chose) next to the quality selector of the preview, the one of the effect costs, so that picking Q0
to Q3 runs the preview at that level, with the values of that level.

*Add preset* adds a preset without a label (`add_preset`) and shows it. A preset shows its id and
label and one row per property it may set (not files, folders, secret texts, buttons, groups, labels
or dividers), with the widget of the kind of the property: a row the preset sets shows its value,
the others the default of the property, and *Default* takes a value out of the preset.

A value that follows a combo or a switch with `{"prop"}`, such as the image of a layer, the texture
of a mask, of an effect or of particles, a video or a color, shows the property it follows, among
the properties that hold a value, and under *Map* one row per option (*Off* and *On* for a switch)
with the widget of the value itself, such as the list of images: the variants of `scene.md`. A row
the map lacks is empty, and the engine then reads the value of the property itself.

Adding, changing or removing properties recomputes `requires.features` like any other step:
`properties2` follows the newer kinds and fields of `scene.md`, `folder` a folder and `user_videos` a file or folder that accepts videos.
