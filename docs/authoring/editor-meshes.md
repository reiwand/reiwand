# Meshes, bones and expressions in the editor

These pages explain the scene editor's tools for mesh layers: cutting a mesh from an image, placing bones, painting weights, giving depth, chains that reach a target, physics, blend shapes and expressions, parts, attaching layers and posing a bone. Each task says where it is, the steps and the terminal equivalent. The rest of drawing (shapes, text, maps, sprites and images with depth) is in [editor-drawing](editor-drawing.md).

Before you start:

- The **preview** is the editor canvas. The mesh tools draw their handles and brushes over it.
- Layers are added in the **Layers** tab with **Add a layer**; files are imported in the **Assets** tab with **Import…**.
- Some field names are shown as the scene format names them (for example "Mesh", "Hit", "Position"); here they are quoted as they appear.
- Each stroke and each drag that releases the mouse is a step that can be undone. The shortcuts and the rest of the window are in [editor](editor.md).
- Each action has its equivalent in the terminal with `reiwand-ctl scene act DIR ACTION JSON`; the arguments are in [actions](actions.md).

Meshes in the editor, by task:

- [Meshes and bones](editor-meshes/bones.md): what a mesh is, making one, bones, weights, depth and reaching a target.
- [Bone motion, blend shapes and parts](editor-meshes/motion-and-shapes.md): physics, the pointer, parent rules, blend shapes, order, attached layers and posing.
