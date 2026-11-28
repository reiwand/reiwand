# Adjustment layers, rest rules and cost

On this page: adjustment layers, wake rules, cost and package effects.

## How to use an adjustment layer

**Where it is:** **Layers** tab, **Add a layer** button, **Adjustment** entry.

**Steps:**

1. Click **Add a layer** and choose **Adjustment**.
2. Add effects to the adjustment layer as to any layer (see [How to add an effect to a layer](basics.md#how-to-add-an-effect-to-a-layer)). They apply to everything below it in the layer list.
3. For it to affect only a part, type a **Size** in the layer's inspector. Only that rectangle, around the layer's position, is captured, and the effects change only that part.
4. On the canvas you see what it captures:
   - a filled rectangle, the captured part;
   - a dashed frame, the most it can capture (twice the canvas on each side; it turns red if **Size** exceeds it);
   - a text, “Captures … × … px; the most is … × … px”;
   - handles at the corners and in the middle of the sides: **Size the captured part; the layer keeps its scale**.
5. With **Redraw** the captured part is drawn again, with the layer's position, rotation and opacity, like a magnifier or a mirror; its outline is dashed and green.

An adjustment layer can also send what it captures to the computer's RGB lighting: the **Send what this layer captures to RGB lighting** switch and the **Whole canvas** and **A strip of it** buttons are in its inspector, with a view of what the lights receive.

**In the terminal:** `reiwand-ctl scene act DIR add`; the size with the `layer` action (`transform` with `size`) and the handles with `layer` `grip`; the lighting with the `rgb` action or `reiwand-ctl scene act DIR rgb`. See [actions](../actions.md).

## How to change an effect's wake rules

A scene with nothing changing falls asleep and uses nothing. Each parameter of an effect can ask for the scene to stay awake while it is not zero, wait for another parameter, or yield to it.

**Where it is:** inspector, with the effect chosen, the **Wake** field (the program does not translate it) at the end of the parameters.

**Steps:**

1. Choose the effect. If it is a built-in one, the field only shows its rules; a package effect lets you change them.
2. **Always asks for:** lists what the effect always asks for.
3. For each parameter there are three dropdowns:
   - **Asks for**: **Nothing**, **Time**, **The pointer**, **The sound** or **Clicks**. While the parameter is not zero, the effect asks for that input.
   - **Waits for nothing** or **Waits for** another parameter.
   - **Yields to nothing** or **Yields to** another parameter.
4. The scene sleeps when no parameter asks for anything.

**In the terminal:** the `effect_rule` action; `reiwand-ctl scene act DIR effect_rule` or the `rules` query. See [actions](../actions.md).

## How to see what an effect costs

Each effect costs units of work per frame. The program lowers the quality if the scene is too heavy (on battery, if the computer is slow) and then trims effects: the light ones stay at every level; medium and heavy ones work at a smaller size or are skipped, those marked **Optional** first. The cost of a single-pass effect with `bounded`, or with a fixed image mask in `strength` mode (not inverted, not feathered and with strength 1), counts only the part of the image that its lit pixels cover (with a minimum of 5 % for the pass's fixed cost); that of a multi-pass effect, or of a mask of another kind, counts the whole layer. That of a package effect counts the whole function it calls (a blend with many modes raises each use to about 2 units), and a script that uses the camera makes all layers count as the whole canvas.

**Where it is:** inspector, with an effect chosen, **Cost** section.

**Steps:**

1. Choose the effect. Under **Cost** there is a list with one line per level: “Q1: 0.56 units of work”, or “Q0: left out of the scene at this level”. The one for the preview's level is in bold.
2. **Preview quality** changes the level the preview draws: **Q0, lowest**, **Q1**, **Q2** and **Q3, highest**. **Restart** starts the preview over.
3. The **Preview level** button, in the canvas bar, opens a panel with the same level, **On battery**, **Less motion**, the scene summary (“Drawn as Q3 at 100 % of the size, with 0 effects left out”) and the cost of each effect of the scene, with **Compare with** another level.
4. Effects that follow the pointer are tested with **Test the pointer and clicks**; the panel names them.

**In the terminal:** the `effect_cost` query or `reiwand-ctl scene ask DIR effect_cost`. See [actions](../actions.md).

## How to write a package effect

A package effect is two files in the scene's `effects/` folder: `NAME.json` (the descriptor: parameters, passes, cost) and `NAME.wgsl` (the shader).

**Where it is:** **Assets** tab, in the effects section, below the **Import an effect…** button: the row with **Id of a new effect**.

**Steps:**

1. In **Id of a new effect** type an id with lowercase letters, digits and underscores (up to 32).
2. Choose the template in the dropdown:
   - **A change of color**: a tint fused with the neighboring effects.
   - **A bend around a point**: moves the image around a point you place on the canvas.
   - **Two passes**: a blur along an angle, in two passes.
   - **A material shader**: for 3D model materials (see [How to write a material shader](package-effects.md#how-to-write-a-material-shader)).
   - **A particle shader**: for particle systems (see [editor-particles-and-sound](../editor-particles-and-sound.md)).
3. Click **New effect**. The effect appears in the **Effects** list with its size.
4. To use a layer effect, choose a layer and click **Add an effect**: it is in the list, with the program's ones.

**In the terminal:** `reiwand-ctl scene act DIR shader`, or the `shader` action with `new`. See [actions](../actions.md).

## How to write and test an effect's code

**Where it is:** **Assets** tab, **Effects** list (the list appears when the package has any effect). Each package effect has four buttons: **Edit the WGSL code…**, **Edit the descriptor…**, **Reimport…** and **Remove**.

**Steps:**

1. Click **Edit the WGSL code…** in the effect's row. The code opens in the **Code** panel.
2. Write. Half a second after you stop typing, the editor checks the effect the way the program does when loading it, and the notice at the top says:
   - **Checking…**;
   - **Builds; costs N units**, if everything is fine;
   - **Line L, column C: message**, if there is an error. The line is marked in red in the text, and the **Go to the line** button takes the cursor there.
3. Check **Preview while typing: write each version that builds** to see the change in the preview while you write, on the layers that use the effect. Each version that builds is saved by itself; consecutive steps are undone together.
4. Click **Save** to save by hand; the title shows “(not saved)” while there are changes. It is saved even if the text does not compile.
5. Click **Use on the selected layer** to add the effect to the layer you have chosen (layer effects only).
6. **Open in your editor…** opens the `.wgsl` file with your text editor.
7. **Close** closes the window.

A shader's text cannot exceed 64 KiB. The error messages are written by the checker, as they are.

**Shortcut:** Ctrl+S inside the text saves the shader, even if it does not compile (`StandardKey.Save` in `SceneShaderCode.qml`).

**What to write:** a color template shows the shape of an effect function.

```wgsl
fn color_tint(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let tinted = vec4<f32>(color.rgb * param(1u).rgb, color.a);
    return mix(color, tinted, param(0u).x);
}
```

`param(N)` reads parameter N of the descriptor, counting from 0 in the order of the list. A warp is called `uv_<id>` and a color change `color_<id>`.

**In the terminal:** the `shader` action with `write` (the WGSL text or the descriptor); the `shader_check` query checks the effect, with unsaved text if you give it, and says line, column, cost and handles (`reiwand-ctl scene ask DIR shader_check '{"effect":"NAME"}'`); `shader_source` gives the descriptor and the code. See [actions](../actions.md).
