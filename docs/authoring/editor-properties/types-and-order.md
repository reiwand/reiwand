# Other types and order

On this page: the other types, order, groups, renaming and removing.

## How to set up the other types

Each type shows in the inspector only the fields that apply to it. All have in common **Label**, **Help**, **Group**, **Order**, **Icon**, **Shown when** and **Enabled when**.

- **Switch**: **Default**, on or off. It also has **Values by quality level**.
- **Color**: **Default** with the color picker. **Transparency** lets the user also choose the opacity; without it the color is opaque.
- **Choice**: the **Options** list, with the **Add** button. Each option has a value and a label; an option with an empty label shows its value. **Add** puts an option with an empty label and a value that no other has (`option_3`...). The **Default** is a box with the options, named by their label. There is a maximum of 64 options, all of the same value type (texts, numbers or yes and no).
- **Text**: **Default**, **Maximum length** (from 1 to 1024 characters), **Several lines** and **Secret**. A secret text has no default, the system stores it apart and the scene sees it empty; it is only received by content that you have given network permission.
- **File**: **Accepted files** (**Image** and **Video** checkboxes) and **Default asset**, the image or video of your package that is used while the user has not chosen another.
- **Folder**: **Accepted files**, **Pick the files** (**At random** or **In order**), **Every** (the seconds between one file and the next, from 10 to 86400), **Most files** (from 1 to 1000), **What the content may read** (**One file at a time** or **All files**, the latter only matters to web pages) and **Subfolder levels** (from 0 to 2).
- **Shortcut**, **Button**, **Label** and **Divider**: they have no fields of their own. A **Divider**'s label can be left empty; then only the line is seen. A **Button** is explained in [How to use a button as a trigger](behavior.md#how-to-use-a-button-as-a-trigger).
- **Group**: it has **Collapsed**, so that it is born closed in the user form.

For a user's file or folder to appear in the scene, link a layer's value to the property (see [How to make a layer follow a property](behavior.md#how-to-make-a-layer-follow-a-property)). A file chosen by the user also replaces the **Default asset** wherever the scene uses it.

In the terminal: `set` on `/manifest/properties/ID/FIELD`, for example the default value, the options or a text's limit. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"weather"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/weather/options","value":[{"value":"sun","label":"Sun"},{"value":"rain","label":"Rain"}]}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/weather/default","value":"sun"}'
reiwand-ctl scene act my-scene add_property '{"kind":"text","id":"name"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/name/max_len","value":20}'
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"tint"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/tint/alpha","value":true}'
reiwand-ctl scene act my-scene add_property '{"kind":"folder","id":"photos"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/photos/every","value":120}'
```

## How to order and group properties

Where it is: each row of **User properties** has the **Raise**, **Lower** and **More** buttons. The **More** menu has **Rename…**, **Move to group** and **Remove**.

To order:

1. Click **Raise** or **Lower** on the row. The property moves among those of its same group (or among those with no group). **Raise** is not active on the first one and **Lower** is not on the last one.

To group:

1. Create a property of type **Group** and give it a label.
2. On another property's row click **More**, **Move to group** and choose the group. The property becomes the last in that group and the row is indented under it.
3. To take it out, **More**, **Move to group**, **No group**. It is left right after its group.

You can also choose the group in the inspector's **Group** field, which offers the groups with their label. A group cannot be inside another, so for a group the **Move to group** menu is not active. If you give a group a condition, it applies to all its members.

The inspector's **Order** field is a free number: properties are ordered by it and, if they tie, by id. **Raise** and **Lower** rewrite it by themselves; there is no need to touch it by hand.

In the terminal: the `move_property` action (command `reiwand-ctl scene act DIR move`, with `--group`; an empty group takes it out of the group). The position is among the group's properties, 0 the first. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"group","id":"look","label":"Look"}'
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"glow"}'
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"tint"}'
reiwand-ctl scene act my-scene move_property '{"id":"glow","to":0,"group":"look"}'
reiwand-ctl scene act my-scene move_property '{"id":"tint","to":1,"group":"look"}'
reiwand-ctl scene act my-scene move_property '{"id":"tint","to":0,"group":""}'
```

## How to rename or remove a property

Where it is: the property's row, the **More** button, **Rename…** and **Remove** entries.

To change the id:

1. Click **More** and **Rename…**. The **Rename the property** dialog opens.
2. The dialog warns that links, triggers, groups, conditions and presets follow the chosen id and that scripts are not rewritten. If any script names the property in quotes, the dialog lists it: “These scripts name it and are not rewritten: ...”. You fix those scripts yourself.
3. Type the chosen id and confirm the dialog. The inspector stays on the property with the chosen id.

To remove it:

1. Click **More** and **Remove**. If any script names it, **Remove the property** opens first with the list of scripts; confirm the dialog to continue.
2. The property disappears together with its values in the presets and its condition. If it was a group, its members are left with no group. If you had it chosen, the inspector goes to the package data.
3. If a layer, a trigger or a condition still names it, the editor does not remove it and shows an error that says what names it (“... is still named by ...”). Change that link first and try again.

In the terminal: the `rename_property` and `remove_property` actions (commands `reiwand-ctl scene act DIR rename` and `scene act DIR remove`). See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed"}'
reiwand-ctl scene act my-scene rename_property '{"id":"speed","to":"pace"}'
reiwand-ctl scene act my-scene remove_property '{"id":"pace"}'
```
