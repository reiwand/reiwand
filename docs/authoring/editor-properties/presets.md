# Translations and presets

On this page: translations, package presets, testing as the user and video presets.

## How to translate labels and texts

Labels, helps, option labels and preset labels are texts that can be in several languages. In the inspector these fields have a row per language.

Where it is: the inspector of a property, a preset or the package, in each text field (**Label**, **Help**...).

Steps to add a language:

1. Below the rows there is a box with the help text **Language, such as es or pt-BR**. Type the language code (two or three letters, with region if needed: `de`, `pt-BR`).
2. Click **Translate**. A row is added for that language, starting from the text of the package's main language. **Translate** never overwrites a text that already exists.
3. Type the row's text. It is saved when you leave the box.
4. To remove a language, click **Remove this language** at the end of its row. An empty text also removes it. A title's last text cannot be removed.

Until you add a second language, the field has a single text, in the main language. Whoever uses the wallpaper sees the text in their language and, if it is not there, the one in the main language. The package's main language is chosen in the package data (**Default language**, see [package](../package.md)).

If the package uses the old data format, the notice appears with the **Update** button and texts in several languages wait until you use it.

### Texts by key

A text that starts with `@` takes its words from a table, instead of writing them in each place. That way a sentence is translated once and used in many places.

Where it is: **Package details**, **Translated texts** field.

The table shows:

- The list of languages. Each language has an **In its own file** switch that stores its texts in `locale/LANGUAGE.json` instead of in the package data.
- Each key such as `@greeting`, with a box per language. Those that are missing are seen in red with the text **Missing**, and above there is a summary (“%1 texts are missing in some language.” or “Every key has a text in every language.”). A key that no text uses carries **not used**. The **Remove this key** button deletes the key in all languages.
- A row at the end to add a key: the **New key, such as greeting** box, a box with the language, the **Its text** box and the **Add** button. The key can have letters, digits, `_`, `.` and `-` (up to 64) and cannot start with `reiwand.`.
- **Texts of the engine**: the `reiwand.*` keys, which the engine already ships translated into English and Spanish.

To use a key:

1. Go into the text field (a label, a title...).
2. Click the button next to it, **Use a text by key**. It offers the keys of your table and those of the engine.
3. Choose one. The field now holds `@key`.

In the terminal: `translate` (command `reiwand-ctl scene act DIR translate`, with `--add` and `--remove`) for a text; the `manifest` action with `text` for a table key and with `locale` to store a language in its file. The `strings` query shows the table. See [actions](../actions.md).

```sh
reiwand-ctl scene new my-scene --locale de
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed","label":"Geschwindigkeit"}'
reiwand-ctl scene act my-scene translate '{"pointer":"/manifest/properties/speed/label","language":"en","text":"Speed","add":true}'
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"de","text":"Hallo"}}'
reiwand-ctl scene act my-scene manifest '{"text":{"key":"greeting","language":"en","text":"Hello"}}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/properties/speed/help","value":"@greeting"}'
reiwand-ctl scene ask my-scene strings '{}'
reiwand-ctl scene act my-scene manifest '{"locale":{"language":"en","file":true}}'
```

## How to make a package preset

A preset gives values to several properties at once. Whoever uses the wallpaper chooses it in the **Presets** list of the scene's settings.

Where it is: **Options** tab, under the **Presets** title.

Steps:

1. Click **Add a preset**. One is added with id `preset_1` (the next free number), empty, and left chosen in the inspector.
2. In the inspector set the **Label** (with languages, like any text). The **Id** can be changed if you want.
3. Below is **Values**. There is a row for each property a preset can set: files, folders, shortcuts, secret texts, buttons, groups, labels and dividers are not there. Each row uses its property's control. A row that the preset sets shows its value; the others show the property's default.
4. Change the rows you want: the preset sets only those.
5. For a property to stop being part of the preset, click **Default** on its row.
6. To delete the preset, click **Remove** on its row in the list.

Presets are saved with the rest of the package when you save the folder (see [editor](../editor.md)). If you rename or remove a property, its values in the presets are renamed or disappear with it. A value that does not fit the property (out of range, an option that no longer exists) appears in the problems list, pointing to the preset's specific value.

What the user can do with them afterwards is not your concern: they choose one, **Save as preset…** the current values with a name of their own, and can rename, delete, export and import their own. User presets do not touch the package's.

In the terminal: the `add_preset` action (command `reiwand-ctl scene act DIR add`, with `--id` and `--label`) and `set` to fill it. See [actions](../actions.md).

```sh
reiwand-ctl scene act my-scene add_property '{"kind":"slider","id":"speed"}'
reiwand-ctl scene act my-scene add_property '{"kind":"color","id":"tint"}'
reiwand-ctl scene act my-scene add_preset '{"id":"calm","label":"Calm"}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/presets/0/values/speed","value":2}'
reiwand-ctl scene act my-scene set '{"pointer":"/manifest/presets/0/values/tint","value":"#3d5aff"}'
reiwand-ctl scene act my-scene remove '{"pointer":"/manifest/presets/0/values/tint"}'
```

## How to test the properties as the user will see them

Where it is: left panel, **Try** tab.

The tab says: **Try the settings of the package in the preview. Only the preview shows them: nothing is saved and no screen changes.** It is the same form that whoever uses the wallpaper sees, with the same controls, the same order, the same groups and the same conditions, and with the labels in your desktop's language.

Steps:

1. Open the **Try** tab. If the package has no properties, it says **The package declares no properties to try.**
2. Move a slider, change a color, click a switch. The preview shows it right away. Fields that depend on a condition appear and disappear as they will for the user.
3. If the package has presets, choose one in the **Preset** box (the first entry, **Package defaults**, is no preset). The controls take their values.
4. To leave the test, click **Back to the defaults**.

What you test is not saved in the package nor does it change any real screen. To test the quality levels use the preview's quality selector (see [How to change a value by quality level](behavior.md#how-to-change-a-value-by-quality-level)).

In the terminal: `reiwand-ctl scene preview DIR` with `--values FILE.json` (an object such as `{"speed": 2}`) and `--preset ID`, which starts from a preset before applying the values. Nothing is saved. The desktop daemon is needed. See [cli](../cli.md).

## Video presets

In a video package, the **Presets** panel (on the left of the **Wallpaper** layout) lists the video's presets, with **Add a preset** and **Remove**. A video preset sets with one click the speed, the part that plays, the volume, the zoom and the visible part. It is filled in in the inspector when you choose it. The package data (title, license, preview) are in [package](../package.md).
