# Drawing in the editor: shapes, text, maps, sprites and depth

These pages explain the scene editor's tools for drawing and giving shape: shapes with their gradient, text with its font, painted maps, sprites and the depth of images. Meshes with bones have their own page: [editor-meshes](editor-meshes.md). Each task says where it is, the steps, and the terminal equivalent.

Before you start:

- The **preview** is the editor canvas, where the layers are seen. Almost all the tools on these pages draw their handles and brushes over it.
- In the **Scene** layout, **Assets** and **Scripts** are tabs on the left, and **Layers** is on the right, under **Inspector**. Layers are added with **Add a layer**, in **Layers**; files are imported in **Assets**.
- When you choose a layer, the inspector shows its fields. Some field names are shown as the scene format names them (for example "Radius", "Gradient", "Stroke"); here they are quoted as they appear.
- Each stroke and each drag that releases the mouse is a step that can be undone. How to undo, the shortcuts and the rest of the window are in [editor](editor.md).
- Each action has its equivalent in the terminal with `reiwand-ctl scene act DIR ACTION JSON`; the arguments are in [actions](actions.md). The terminal list is `reiwand-ctl scene actions`.

Drawing in the editor, by task:

- [Shapes, gradients and text](editor-drawing/shapes-and-text.md): shapes, their handles, gradients and texts.
- [Fonts, light maps and sprite sheets](editor-drawing/fonts-and-maps.md): fonts, lit images and their maps, normal maps, sprite sheets and depth tools.
- [Depth maps and cuts](editor-drawing/depth.md): the depth map, its brush, the layer cuts and fills.
- [Regions and the parallax model](editor-drawing/regions.md): regions, how the image moves and downloading the model.
