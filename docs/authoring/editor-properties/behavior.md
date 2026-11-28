# Following properties and conditions

On this page: layers that follow a property, conditions, buttons and quality levels.

## How to make a layer follow a property

Where it is: in a layer's inspector, the small button to the right of each value that can be linked (**◇** if it is fixed). If the value already follows a property, the button shows **⛓** and below it there is another button that says **Follows a user property**, with the property's id after it.

Steps:

1. Choose the layer and find the value (the opacity, the color, whether it is shown, a mask's image...). Only the values that admit a link have the button.
2. Click the ◇ button and choose **Follow a user property**. The submenu lists the properties that store a value (not buttons, groups, labels or dividers), with their label.
3. Choose the property. The value now follows it: the field is replaced by the **Follows a user property · id** button.
4. To change the property or adjust the link, click that button or the **Edit the source…** entry of the menu. The inspector shows the **User property** field (a box with the properties) and, depending on the value, its scale and range settings.
5. To go back to a fixed value, open the button's menu and choose **Fixed value**.

The property type has to match the value: a slider works for numbers, a switch for yes or no, a color for colors, a text for texts, a file or a folder for images and videos. A property that does not exist is a problem in the editor's problems list (see [editor](../editor.md)).

### A different value for each option

A **Choice** or a **Switch** can give a different value of the layer for each option. It is what is used to change image according to an option, for example a layer's image, the texture of a mask, of an effect or of some particles, a video or a color.

1. Link the value to the property as above.
2. The **Map** group appears in the inspector. Below it, a box with the options that have no value yet and the **Add** button.
3. Choose an option and click **Add**. A row appears with that option (on a switch, **On** and **Off**) and the control for the value itself: the list of images, the color picker, a switch.
4. Set each row's value.

An option without a row keeps the property's value as the user chooses it. To remove a row use the **Default** button of its field.

In the terminal: `set` with the value's pointer and `{"prop": "ID"}`, with `"map"` if each option gives a value. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"weather"}'
reiwand-ctl scene act my-scene add_layer '{"kind":"solid","id":"sky"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/visible","value":{"prop":"weather","map":{"option_1":true,"option_2":false}}}'
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"light"}'
reiwand-ctl scene act my-scene set '{"pointer":"/scene/layers/0/opacity","value":{"prop":"light"}}'
```

## How to set conditions: show and enable

Where it is: the property's inspector, **Shown when** and **Enabled when** fields. Both are a text box; empty it shows **Always**.

- **Shown when** hides the property while the condition is not met.
- **Enabled when** leaves the property visible but off (it cannot be touched) while the condition is not met.

Steps:

1. Choose the property that must be hidden or turned off (one of those that depend on another).
2. Type the condition in **Shown when** or in **Enabled when**.
3. While you type, the editor checks it at every letter. If it fails, the reason appears in red below the box and nothing is saved. It is saved when you leave the box or press Enter, only if the condition is valid.
4. To remove it, empty the box.

Three ways to write it:

- An expression: ids of other properties, numbers, texts in single quotes, `true` and `false`, with `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!` and parentheses. For example `weather == 'sun'` or `mood == 2 || speed > 1.5`. The box's help (when you hover over it) reminds you of it.
- A map of values such as `{"weather": "sun"}`, which holds if all of them match.
- Nothing, for it to always hold.

Rules: an expression has at most 256 characters and 8 levels of parentheses. A property is only compared with a value of its same type, and `<` and the like only with numbers. The editor rejects ids that do not exist, properties that store no value (a button, a group), conditions that do not give yes or no and conditions that depend on themselves through other properties.

If you give a group a condition, it applies to all its members.

In the terminal: the `condition` action (command `reiwand-ctl scene act DIR condition`, with `--enabled` for **Enabled when**; an empty text removes it). See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"combo","id":"weather"}'
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"wind"}'
reiwand-ctl scene act my-scene condition '{"id":"wind","text":"weather == '\''option_2'\''"}'
reiwand-ctl scene act my-scene condition '{"id":"wind","text":"weather != '\''option_1'\''","enabled":true}'
reiwand-ctl scene act my-scene condition '{"id":"wind","text":""}'
```

## How to use a button as a trigger

A **Button** stores no value. Clicking it is a notice that can start timelines and sounds.

Steps:

1. Add a property of type **Button** and give it a label.
2. Choose the timeline (or the sound layer) that should start with it. Its **Trigger** field (on sounds, **Play on**) is a box that offers, among other things, each button and each property that stores a value, with their label. Choose yours. See [animation](../editor-animation.md) and [particles and sound](../editor-particles-and-sound.md).
3. To the right of the box, the **Fire this trigger in the preview** button does the same as the user's button without leaving the editor. Its help says which timelines and sounds start (“Fires ... in the preview, as the scene would, starting ...”), or that nothing starts yet.
4. Choose **Not set** in the box so that nothing fires it.

A button cannot be clicked while its **Enabled when** condition, or its group's, is not met. The field also names the properties that store a value, not only the buttons.

In the terminal: the trigger is set with `timeline` (field `trigger`, with the value `prop:ID`). The `fires` query says what a trigger starts. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"button","id":"jump"}'
reiwand-ctl scene act my-scene timeline '{"add":true,"id":"hop","duration":1,"trigger":"prop:jump"}'
reiwand-ctl scene ask my-scene fires '"prop:jump"'
```

## How to change a value by quality level

Where it is: the inspector of a **Slider**, a **Switch** or a **Choice**, **Values by quality level** group. The other types do not change with quality and do not show it.

It is for an energy saving to turn on by itself: at low levels the scene sees another value (on a slider, a cap; on a choice or a switch, exactly that value), even if the user chose another. The stored value does not change.

Steps:

1. Choose the property.
2. In **Values by quality level** there is a row per level: **Q0, lowest**, **Q1**, **Q2** and **Q3, highest**. Each row uses the property's control (the slider with its range, the options box, the switch).
3. Set the value of the levels you want. A level with its own value holds for it and for the lower ones until another level sets its own.
4. To remove a level's value, click **Default** on its row.
5. Further down, **Value in effect at every level** lists what each level ends up with (“Q1: 3”, or “Q2: 3 (inherited)” if it takes it from another). The level the preview uses is in bold.
6. With the quality selector in that area, you choose Q0 to Q3 and the preview is seen with that level's values.

In the terminal: `set` and `remove` on `/manifest/properties/ID/quality/q0` (up to `q3`). See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"particles"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/particles/quality/q0","value":10}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/particles/quality/q1","value":30}'
reiwand-ctl scene act my-scene remove '{"pointer":"/manifest/properties/particles/quality/q0"}'
```

To see the preview at a level from the terminal there is `reiwand-ctl scene preview DIR --quality q1` (see [cli](../cli.md)).
