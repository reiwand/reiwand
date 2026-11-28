# Package effects and shaders

On this page: descriptors, importing effects, material shaders and the image editor.

## How to edit the descriptor of an effect

**Where it is:** **Assets** tab, **Effects** list, the effect's **Edit the descriptor…** button.

**Steps:**

1. The **Descriptor of NAME** window opens. At the top, the notice says **Builds** or the error.
2. In **Parameters** there is a row per parameter, from left to right:
   - the id;
   - the type, with its English name (`float`, `color`, `texture`, `position`…);
   - **Default**, which is typed as JSON, and **Min** and **Max** (empty removes the limit);
   - **Degrees**, on numbers: the parameter is measured in degrees and gets a dial;
   - on image or mask parameters, which tools it offers: **Tools by its id**, **Gradient stops**, **Grey brush**, **Grey and direction brushes** or **Grey brush and depth map**;
   - which handle it receives on the canvas: “a point on the canvas”, “a corner on the canvas”, “an arrow on the canvas” or “a dial on the canvas”, with the parameter it pairs with;
   - a button to raise it in the list and another to remove it.
3. To add a parameter, type its id in **Id of a new parameter**, choose the type and click **Add the parameter**.
4. **Passes** (only on layer effects): a row per pass with **Entry point**, its scale in percent, the **Inputs** (“previous”, “pass:0”, “target:soft”…) and the **Target**. **Add a pass** adds another.
5. **Declared cost: taps, passes and scale**: three numbers with what the effect asks for per frame.
6. For a layer effect:
   - **Mask**: **Blend the result by the mask**, or **The shader scales itself by mask_at**.
   - **Fused as**: **Not fused**, **A bend of the picture (uv_)** or **A change of color (color_)**.
   - **Grows the layer by**: **Nothing** or a numeric parameter.
7. **Cost when a layer marks it optional** recomputes the line below, **Cost per quality level**, which gives the units of each level or “left out”.

Each change is saved and is an **Undo** step. What the form does not change (the category, the description and the label and help of each parameter) is in the `effects/NAME.json` file: edit it with your editor, or from the terminal with the `shader` action (`param` with `label` and `help`, `field` with `category` and `description`).

**In the terminal:** the `shader` action with `param`, `pass` and `field`; `shader_source` and `shader_check` to read and check it. See [actions](../actions.md).

## How to import, reimport and remove a package effect

**Where it is:** **Assets** tab, **Effects** list.

**Steps:**

1. **Import an effect…** opens a file dialog: choose the `.json` descriptor and, if it is not next to it, the `.wgsl` shader.
2. **Reimport…** in an effect's row does the same and replaces the one you have.
3. **Remove** deletes the effect from the package. It is refused while any layer uses it.

**In the terminal:** `reiwand-ctl scene act DIR import_effect`, `scene act DIR remove`; the `import_effect` and `remove_effect` actions. See [actions](../actions.md).

## How to write a material shader

A material shader changes how the surface of a 3D model looks. It is written as a package effect, with the material template.

**Where it is:** a model layer's inspector, **Materials** section, at the end. The section only appears if the model has materials. See [editor-3d](../editor-3d.md).

**Steps:**

1. Type the name in **Name of a new material shader** (lowercase, digits and underscores) and click **New material shader**.
2. Below the section a row appears with the shader's state: **NAME builds**, or the error with its line and column.
3. In the row there are three actions: **Edit the WGSL code…**, **Edit the descriptor…** and **Check again**. The code and the descriptor are edited like those of an effect.
4. On each material, the shader dropdown lists the package's ones with their id. Choose yours.
5. The shader's parameters appear below: a slider for a number, a picker for a color, a checkbox for a yes or no and, for an image, a list with the package's images and `white`, `black`, `noise` and `normal_flat`.
6. A notice reminds you that the shader runs on the graphics card within the limits of the package's effects, and that you should only use shaders you trust.

The shader writes `fn material(s: Surface) -> Surface` in `effects/NAME.wgsl`. It reads `frame`, `param(0u)` to `param(7u)` and up to four images from its texture parameters with `image0(uv)` to `image3(uv)`.

```wgsl
fn material(s: Surface) -> Surface {
    var o = s;
    let tint = param(0u) * image0(o.uv);
    o.albedo = vec4<f32>(o.albedo.rgb * tint.rgb, o.albedo.a);
    o.emissive = o.emissive + o.albedo.rgb * param(1u).x * (0.5 + 0.5 * sin(frame.time));
    return o;
}
```

**In the terminal:** `reiwand-ctl scene act DIR scene3d` (it also checks a shader and prints its errors) or the `scene3d` action with `new_material` and `material`; the `material_check` query. See [actions](../actions.md).

## Effects in the image editor

The app's image editor (opened with **Edit…** on the card of a library image) has its own **Effects** and **Masks** tools, separate from the scene's. They do not edit scenes: they edit the image without touching its file.

### How to blur, add a vignette or overlay a color

**Steps:**

1. In the list on the left, choose **Effects**.
2. **Blur**: the **Radius** slider.
3. **Vignette**: **Amount** (negative darkens, positive lightens the edges), **Midpoint**, **Roundness** and **Feather**.
4. **Color overlay**: **Choose a color…**, the blend mode (**Normal**, **Multiply**, **Screen**, **Overlay**, **Soft light**, **Color**) and **Opacity**.

**In the terminal:** `reiwand-ctl edit blur`, `edit vignette` and `edit overlay`.

### How to limit an edit with a mask

**Steps:**

1. In the list on the left, choose **Masks**.
2. Type the **Name of the mask** and choose the type: **Linear gradient**, **Radial gradient**, **Brightness range**, **Depth range** or **Painted with a brush**.
3. Adjust the type's sliders. On the linear and radial gradients you can also drag the handles over the image: “Drag the handles on the picture to place the mask.”
4. With **Painted with a brush** name the mask and paint over the image. **Brush size** and **Hardness** control the brush; **Erase** removes what was painted. Each stroke is an **Undo** step.
5. **Invert** flips the mask.
6. The masks made appear as chips above the name: click them to edit them, and the chip's cross removes them.

“Limit an effect to part of the picture with a mask in the list of operations.”

**In the terminal:** `reiwand-ctl edit mask`.
