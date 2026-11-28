# Making a preset package

On this page: a package that is a preset of another.

## How to make a preset package

A preset package brings no scene: it gives values to the properties of another package that the user already has installed. It is the way to share nice settings of someone else's scene.

Where it is: the app's **Scenes** page has the **Preset** starting point. When you create it, the editor opens the package and, in its data, the **Preset** section appears instead of **Required packages**.

Steps:

1. Create the package from the **Preset** starting point.
2. In the **Preset** section, choose the **Base package** in the box (it lists the installed packages; you can also type its id by hand).
3. Type the **Version range** it is compatible with (for example `^1.2`). If the base package is installed, the box suggests “Compatible with” its version and you can leave it empty.
4. Click **Use as base**. The line above now says “Preset for ID, version RANGE”.
5. If the base package is installed, below there appears a control for each of its settings, with the same shape as in its form: sliders, colors, switches, option boxes and texts. The settings this preset sets are in bold and show their value; the others show the base package's default. Change the ones you want. **Default** on the right of a row returns that setting to the base package.
6. Image or video settings have a **Picture or video…** button to choose a file, which is copied into the package. A row without a file says **No file**.
7. If the base package is not installed, the editor says “Install the base package to choose the settings this preset sets.” Install it or fill in the values from the terminal.

Publishing as a preset: below the settings, the **Publish as a preset** section lists the steps that are missing, each with a green mark or a notice: the chosen base package, some value or file, the title, the license, the preview image and a review with no problems. **Edit…** opens the package data for the title and the license. For the preview, **Capture the thumbnail** draws the installed base package with the preset's values and saves that frame; **Record a preview** records a few seconds as a gif, or as a webp with the **WebP** checkbox; **Choose a picture…** uses a file of yours. The thumbnail and the recording are drawn without sound or pointer: a scene that reacts to them comes out at rest. Videos that the preset sets as a value are not drawn in the thumbnail either. When all the steps are done, **Publish…** opens the build dialog, which packages, signs, adds the package to a static repository and publishes it. It is the same dialog as for any package: it asks to build, sign and choose a repository; there is no page only for presets. Only a scene's preset can be drawn; that of a web page, an image or a video uses an image you choose.

The title, the license, the origin and the preview image are those of any package (see [package](../package.md)). A preset package without a valid base package has a problem in the list until you choose one, and a package cannot be its own base. Building, signing and publishing are those of any package.

Values the base package does not know are left out, with a notice, when the preset is applied.

In the terminal: the `manifest` action with `preset` (fields `base`, `values`, `files` and `clear`), the command `reiwand-ctl scene new DIR --from preset` (with `--base`, `--range`, `--values`, `--license` and `--preview` to start with everything done), the `shot` action with `base` for the preview and the `preset_steps` query to see what is missing. A simple version such as `1.2.3` becomes the compatible range `^1.2`. See [actions](../actions.md).

```sh
reiwand-ctl scene new my-preset --from preset
reiwand-ctl scene act my-preset manifest '{"preset":{"base":{"id":"org.example.my-scene","version":"1.2.3"},"values":{"tint":"#3d5aff","speed":0.4}}}'
reiwand-ctl scene act my-preset manifest '{"preset":{"clear":["speed"]}}'
```
