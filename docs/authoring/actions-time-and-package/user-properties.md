# User properties

User properties are the settings the person using the wallpaper can change (sliders, switches, colors...). They are stored in `/manifest/properties`. In the app, the package's property editor.

## `add_property`

Adds a new property, at the end of its group. It starts valid, with default values. It prints its id.

| Argument | Type | What it means |
|-----------|------|---------------|
| `kind` | text (required) | Type: `slider` (a slider from 0 to 100), `bool` (switch), `color`, `combo` (a list with two options, `option_1` and `option_2`, with an empty label), `text`, `file` (a file), `folder` (a folder), `shortcut` (a shortcut), `button`, `group` (groups others), `label` (a text) or `divider` (a line). |
| `id` | text | Id. Default `type_1`, `type_2`... |
| `group` | text | Id of a group (another property of type `group`) to put it in. |
| `label` | text | Label the user sees. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"group","id":"lights"}'
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow","label":"Glow","group":"lights"}'
reiwand-ctl scene act my-scene add_property '{"kind":"bool"}'
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"tint"}'
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"mode"}'
reiwand-ctl scene act my-scene add_property '{"kind":"text"}'
reiwand-ctl scene act my-scene add_property '{"kind":"file"}'
reiwand-ctl scene act my-scene add_property '{"kind":"folder"}'
reiwand-ctl scene act my-scene add_property '{"kind":"shortcut"}'
reiwand-ctl scene act my-scene add_property '{"kind":"button"}'
reiwand-ctl scene act my-scene add_property '{"kind":"label"}'
reiwand-ctl scene act my-scene add_property '{"kind":"divider"}'
```

## `move_property`

Moves a property to another place within its group, or to another group.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the property. |
| `to` | integer (required) | Position within the group (0 is the first). |
| `group` | text | Id of the destination group. An empty string takes it out of its group. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene act my-scene add_property '{"kind":"bool","id":"active"}'
reiwand-ctl scene act my-scene move_property '{"id":"active","to":0}'
```

## `rename_property`

Changes a property's id and fixes everything that names it: bindings, `prop:` triggers, groups, conditions and preset values. Scripts are not rewritten: the command warns about each script that names the old id.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Current id. |
| `to` | text (required) | New id. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene act my-scene rename_property '{"id":"glow","to":"light"}'
```

## `remove_property`

Removes a property, its values in the presets and in the conditions, and its membership in a group. It is rejected, saying where, while a binding, a `prop:` trigger or a condition expression names it.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the property. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene act my-scene remove_property '{"id":"glow"}'
```

## `property`

Adjusts a property that already exists: its range, its step, its default value and its place. It rejects a default value that is out of range or does not fall on a step.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the property. |
| `min` | number | Minimum. Sliders only. |
| `max` | number | Maximum. Sliders only. |
| `step` | number | Step. Sliders only. |
| `default` | any JSON value | Default value, of the property's type. |
| `order` | integer | Place within its group, like `to` of `move_property`. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"light"}'
reiwand-ctl scene act my-scene property '{"id":"light","min":0,"max":10,"step":0.5,"default":3}'
reiwand-ctl scene act my-scene property '{"id":"light","order":0}'
```

## `condition`

Decides when a property is shown or when it can be changed, depending on the value of others. It rejects a condition that cannot be checked (an id that does not exist, a comparison between things of different types, one that depends on itself).

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text (required) | Id of the property. |
| `text` | text | The condition. A short expression with property ids, numbers, texts in quotes, `true`, `false`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `\|\|`, `!` and parentheses (for example `active && mode == "option_1"`), or a JSON object written as text, such as `{"mode":"option_2"}` (everything must match). Empty removes the condition. Default empty. |
| `enabled` | yes/no | `false` sets the condition for when it is shown; `true`, the one for when it can be changed (if it is not met, it appears turned off). Default `false`. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"bool","id":"active"}'
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"mode"}'
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"level"}'
reiwand-ctl scene act my-scene condition '{"id":"level","text":"active && mode == \"option_1\""}'
reiwand-ctl scene act my-scene condition '{"id":"level","text":"{\"mode\":\"option_2\"}","enabled":true}'
reiwand-ctl scene act my-scene condition '{"id":"level","text":""}'
```

## `add_preset`

Adds a preset: a set of property values that the user can choose all at once. It starts with no values. It is filled in with `set` on `/manifest/presets/N/values/ID`. It prints its id.

| Argument | Type | What it means |
|-----------|------|---------------|
| `id` | text | Id. Default `preset_1`, `preset_2`... |
| `label` | text | Name the user sees. |

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"light"}'
reiwand-ctl scene act my-scene add_preset '{"id":"soft","label":"Soft"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/presets/0/values/light","value":2}'
```
