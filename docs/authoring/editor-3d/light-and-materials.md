# Light, shadows and materials

On this page: volumetric, ambient and sky light, lit layers, shadows and materials.

## How to make light with air, shafts and volume

Where it is: the inspector of a point light or a spot, **Light the air** part.

Steps:

1. Turn on **Light the air**.
2. Adjust **Density**, **Samples** and **Shafts**. With **Shafts** the air darkens behind the dark parts of the layers below, and a dashed line in the graph shows the glow over one of those dark parts. Layers above the light cover it.
3. The graph draws the glow against the distance.
4. Look at the lines per quality level: **off** in Q0, **half** or **a quarter** of the size with their samples. Phones and the oldest computers use a quarter.

The preview shows the glow live with the quality level chosen in the **Lights** panel.

In the terminal: the `light` action with `volumetric`; query `air`.

## How to give ambient and sky light

Where it is: canvas, **Lights** panel.

Steps:

1. Change the color of **Ambient light** (the one that arrives from everywhere).
2. Change the color of **Sky light** (the one that arrives from above).
3. The sphere next to it is lit with both colors so you can see the effect.

In the terminal: the `ambient` action; query `hemisphere`.

## How to make a layer receive light

Where it is: an image, video or text layer, **Lighting** part. Particles also have the part, with the switch and **Glow** and **Strength**.

Steps:

1. Turn on **Take the light of light layers**.
2. Choose, if you need them, the maps: **Normal map**, **Reflected picture**, **Roughness map** and **Glow map** (**Not set** removes the map). If an image is read as colors and shades badly, a notice appears: choose it again to read it as a map.
3. Adjust the **Roughness**, **Reflection**, **Glow** and **Strength** sliders.

In the terminal: the `lighting` action.

### Generate a normal map

1. With the layer chosen, click **Generate normals…**.
2. In the dialog (**Generate normals from the picture**, **from the frame** or **from the letters**) adjust **Strength** and **Blur**. The letters of a text come out in the same lines as on the canvas (with their wrapping width and their line breaks), and if the text has a background, the background comes out as a plate lower than the letters. You see the relief lit from the upper left, the map itself and a lit sphere before saving.
3. Click **Generate**. The map is left as an image of the scene.

In the terminal: the `normals` action.

### Paint roughness and glow

1. In the **Lighting** part, click **Paint roughness on the preview** or **Paint glow on the preview**.
2. Adjust **Brush size**, **Hardness** and **Gray** (the value that is painted).
3. Paint on the layer in the preview. While you paint, the map is seen on top as a heat map.

In the terminal: the `paint_map` action; query `map`.

Particles have their own light and refraction options; see [editor-particles-and-sound](../editor-particles-and-sound.md).

## How to give shadows to lights

Where it is: canvas, **Lights** panel, **Shadows** block.

Steps:

1. In the row of each point light, spot or directional, turn on **N casts shadows**.
2. Choose the size of the shadow map: **Automatic** or from 256 to 4096 px. The text next to it says what size each equipment level uses, on phones and in Q0.
3. Adjust **Bias** if you see stripes or detached shadows.
4. Click **Show where shadows fall and the frusta of the maps**: in the preview the lit places are seen white and the shadow ones red, and the authoring view draws the volumes of the maps with dashed lines.

The block warns how many lights can cast shadows at most.

In the terminal: `scene3d` with `shadows`; query `shadows3d`.

## How to place a light inside the 3D scene

A normal light stays on the canvas plane. In 3D it has its own point among the models.

Where it is: canvas, **Lights** panel, **Lights in 3D** block: an **N in 3D** checkbox per light.

Steps:

1. Turn on the light's checkbox. Uncheck it to return it to the plane.
2. In the authoring view the light has a knob where it lights the models (dimmed while it is on the plane: dragging it puts it in 3D). Spots and directional lights also have an **Aim** square, the point they aim at, with a dashed line between the two.
3. Drag the knob or the square in the plane of the view; with Ctrl, along the line of sight.

If the position is animated, a key is put at the current time.

In the terminal: `scene3d` with `light` (`place`, `flat`, `position`, `aim`) and with `grab` (`what` `light`).

## How to adjust a material

Where it is: the model layer's inspector, **Materials** part. There is one row per material in the file.

What each row brings:

- A ball lit with the material and with the scene's ambient and sky light.
- A list with the material type: **Physical**, **Unlit**, **Fur**, **Foliage**, **Chroma key** and the package's material shaders.
- Checkboxes **Lit**, **Both sides**, **Casts shadows** and **Receives shadows**.
- Colors **Tint** and **Glow**.
- Sliders **Metal**, **Roughness**, **Relief** and **Occlusion**.
- The thumbnails of the file's maps: **Color**, **Metal and roughness**, **Normals**, **Occlusion** and **Glow**.

Steps:

1. Choose the model.
2. In **Materials**, find the material's row and change what you need.

In the terminal: `scene3d` with `material`; query `materials3d`.

### Fur type

With the **Fur** type these appear:

- **Layers**, **Length, in hundredths of the model** and **Density**; below, how many layers each quality level draws.
- A **Combing** box: drag to comb the fur over the surface; the **Toward the viewer** slider combs toward the viewer.
- The **Comb on the model** brush: turn it on and drag over the model in the view. The stroke is drawn with a ring the size of the brush and, when you release, the fur is combed where it passed. Each stroke can be undone. Adjust **Brush size, in pixels** and **Combing strength**; **Erase** undoes the combing where you pass and **Clear the combing** removes it all.

In the terminal: `scene3d` with `comb`. To paint another map of the material there is `paint`.

### Foliage type

It shows **Sway** and **Speed**, and the canvas wind with its arrow.

### Chroma key type

It removes a color from the model. Choose the **Key color**, or click **Pick the key color from the model** and click on the model in the view. Adjust **Tolerance** and **Feather**.

In the terminal: `scene3d` with `key` (pick the color of a point) and `material`.

### Debug views

Each material row has the **Debug view** list: **None**, **Sway mask of the vertex colors**, **Matte** (the transparency after the chroma key) and **Where shadows fall**. Only that material shows it, and only in the editor's preview; the wallpaper never shows it.

In the terminal: `scene3d` with `debug`.

## How to write the code of a material shader

Where it is: the model's inspector, **Materials**, below the rows.

Steps:

1. Type a name in **Name of a new material shader** and click **New material shader**. A template is created.
2. The shader appears with a line that says whether it **builds** or the error with line and column.
3. Click **Edit the WGSL code…** to open it in the **Code** panel, or **Edit the descriptor…** for its parameters. **Check again** repeats the check.
4. In the editor, write. While you write, the code is checked and the failing line is marked (**Go to the line**). With **Preview while typing: write each version that builds** each version that builds is saved. **Save** saves and can be undone; **Use on the selected layer** applies it to the layer; **Open in your editor…** hands the file to the desktop's editor.
5. The shader writes `fn material(s: Surface) -> Surface` in `effects/NAME.wgsl`. It reads `frame`, from `param(0u)` to `param(7u)` and up to four images from its texture parameters with `image0(uv)` to `image3(uv)`.
6. In a material's row, choose the shader from the list. Its parameters appear as sliders, color pickers, checkboxes or image lists.

The shader runs on the graphics card within the limits of package effects; use only shaders you trust. More about shader code in [editor-effects](../editor-effects.md).

In the terminal: the `shader` action (`new`, `write`, `param`); queries `shader_source`, `shader_check` and `material_check`.
