# Effects, masks and shaders in the scene editor

These pages explain how to add effects to a layer, limit them with masks, place their points on the canvas, paint the maps they use, and write your own effects and shaders. The editor opens with the scene; the other parts are in [editor](editor.md), [editor-canvas](editor-canvas.md) and [editor-drawing](editor-drawing.md).

Everything you do here is an editor action. In the terminal it is applied with `reiwand-ctl scene act DIR ACTION JSON`; the actions and their arguments are in [actions](actions.md). Each drag on the canvas or on a slider counts as a single **Undo** step, and so does each brush stroke.

Effects in the editor, by task:

- [Adding effects](editor-effects/basics.md): what each thing is, adding effects and arranging them.
- [The effect catalog and parameters](editor-effects/catalog.md): the catalog, parameters and points on the canvas.
- [Masks, maps and glow](editor-effects/masks.md): masks and their handles, painted maps, gradients, depth maps and glow.
- [Adjustment layers, rest rules and cost](editor-effects/adjust-and-cost.md): adjustment layers, wake rules, cost and package effects.
- [Package effects and shaders](editor-effects/package-effects.md): descriptors, importing effects, material shaders and the image editor.
