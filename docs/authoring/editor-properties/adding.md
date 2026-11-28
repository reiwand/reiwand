# Adding properties

On this page: where everything is, the types, labels, help, icons and sliders.

## Where everything is

Where it is: the **Options** panel, on the left of the **Wallpaper** layout (**Ctrl+6**); in another layout, **Window** › **Options** opens it.

The tab has, from top to bottom:

- **Package details**: the row that opens the package data in the inspector (see [package](../package.md)).
- **User properties**: a row per property, in the order whoever uses the wallpaper will see them. Each group is followed by its members, a little further to the right. The row says “label · type”. If you have not given it a label yet, it says the id. If you hover over it, the id is seen.
- **Add a property**.
- **Presets**: a row per preset, with its label (or its id if it has none), and a **Remove** button at the end of the row.
- **Add a preset**.

Everything is in a single scrolling list. When you click a row, the inspector (the right panel) shows the fields of that property or preset. In a web page package, these two lists are the **Options** panel, on the left of the **Wallpaper** layout the package opens in; in a video package that panel is called **Presets** (see [Video presets](presets.md#video-presets)).

There are no keyboard shortcuts for this area.

## What property types there are

**Add a property** offers these twelve types. Each is born with some values that you change afterwards:

| Type | What it is | Born with |
|------|------------|-----------|
| **Slider** | A number between a minimum and a maximum | from 0 to 100, step 1, value 50 |
| **Switch** | Yes or no | off |
| **Color** | A color | white |
| **Choice** | A list of options, one is chosen | two options, `option_1` and `option_2`, without a label |
| **Text** | A text the user types | empty |
| **File** | An image or a video the user chooses | accepts images |
| **Folder** | A folder from which images or videos are taken | accepts images |
| **Shortcut** | Something the user chooses to open when clicking the wallpaper | no fields |
| **Button** | It stores no value; clicking it is a notice | no fields |
| **Group** | Gathers properties under a title | no fields |
| **Label** | A line of text in the form | no fields |
| **Divider** | A line across the form, with an optional title | no fields |

File, folder, shortcut and secret text properties are chosen by whoever uses the wallpaper; neither you nor a preset can set them.

## How to add a property

Where it is: **Options** tab, **Add a property** button.

Steps:

1. Click **Add a property**. A menu opens with the types of the table above.
2. Choose a type. The property is added at the end of the list with an id the editor makes up from the type (`slider_1`, `combo_1`...) and is left chosen in the inspector. It has no label yet, so the list shows the id.
3. In the inspector, type the **Label** (see [How to translate labels and texts](presets.md#how-to-translate-labels-and-texts)). The label is what the user sees; the id is only for you.
4. Fill in the rest of the fields of its type (see the following sections).

The id starts with a letter and continues with up to 63 letters, digits or underscores. It cannot be repeated even if only a capital letter differs. To change it later see [How to rename or remove a property](types-and-order.md#how-to-rename-or-remove-a-property).

Each change of properties recomputes what the package needs (for example, that it uses the newest property types), without you having to do anything. Everything can be undone like the rest of the editor.

In the terminal: the `add_property` action (command `reiwand-ctl scene act DIR add`, with `--id`, `--group` and `--label`). The `shortcut` type is added with `scene act`. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed","label":"Speed"}'
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"tint"}'
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"weather"}'
reiwand-ctl scene act my-scene add_property '{"kind":"shortcut","id":"open"}'
```

## How to set a label, help and icon

Where it is: the property's inspector, **Label**, **Help** and **Icon** fields.

Steps:

1. Choose the property in the **Options** tab.
2. Type the **Label**: the name seen in the user form.
3. Type the **Help** if you want a small text below the control. It is optional.
4. Choose an **Icon** from the list. Only the icons the list offers exist; a package cannot bring its own.

If you set a label and help in several languages, see [How to translate labels and texts](presets.md#how-to-translate-labels-and-texts).

In the terminal: `translate` writes a label or a help; the icon and the rest are set with `set` on `/manifest/properties/ID/icon`. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/icon","value":"weather-clear"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/help","value":"The more, the faster"}'
```

## How to set a slider's range, step and default value

Where it is: the inspector of a **Slider** property. At the top there are four boxes: **Minimum**, **Maximum**, **Step** and **Default**. Further down are **Decimals**, **Whole numbers**, **Unit**, **Number box** and **Logarithmic scale**.

Steps:

1. Choose the slider in the **Options** tab.
2. Type the **Minimum** and the **Maximum**. Both values are accepted as you type them, even if they do not fall exactly on a step.
3. Type the **Step**, how far the slider jumps each time. If you leave it empty, the slider moves according to its decimals.
4. Type the **Default**: what whoever does not touch the setting will see. It is clipped to the range.
5. If you want a tag with the measure (for example “s” or “%”), type the **Unit**.
6. Set **Whole numbers** so only integers are valid, and **Decimals** (from 0 to 6) to set how many decimals are rounded and shown.
7. Set **Number box** so that, besides the slider, there is a box to type the number.
8. Set **Logarithmic scale** if the range spans several powers of ten (for example, from 0.1 to 100). It needs a minimum greater than zero.

The value that reaches the scene is always rounded to the decimals and aligned to the step.

In the terminal: the `property` action (command `reiwand-ctl scene act DIR property`, with `--min`, `--max`, `--step`, `--default` and `--order`). The other fields (`decimals`, `integer`, `unit`, `editable`, `log`) are set with `set`. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed"}'
reiwand-ctl scene act my-scene property '{"id":"speed","min":0,"max":10,"step":0.5,"default":2}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/unit","value":"x"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/decimals","value":1}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/editable","value":true}'
```
