# Scene editor

The scene editor of the app and `reiwand-ctl scene` edit a scene folder: `manifest.json`, the
scene file it names as `entry` (`scene.json`) and the assets, by convention under `assets/`. There
is no editor format: the folder is what `reiwand-ctl pack build` packs and what the engine reads
(`scene.md`). Both share one implementation, `crates/reiwand-editor`, so every action of the app is
also a command, with the same arguments and the same undo history.

The specification, part by part:

- [Document and Operations](editor/document.md): document, operations.
- [Actions](editor/actions.md): actions.
- [Curve editor](editor/actions-curve-editor.md): curve editor.
- [Actions from the command line and Scripts and templates](editor/actions-from-the-command-line.md): actions from the command line, scripts and templates.
- [Package effects and shaders and History](editor/package-effects-and-shaders.md): package effects and shaders, history.
- [Inspector](editor/inspector.md): inspector.
- [Shapes, text and fonts, Hearing, shaping and trying a binding and Sound layers](editor/inspector-shapes-text-and-fonts.md): shapes, text and fonts, hearing, shaping and trying a binding, sound layers.
- [Package details](editor/package-details.md): package details.
- [User properties and presets](editor/user-properties-and-presets.md): user properties and presets.
- [Mesh layers](editor/user-properties-and-presets-mesh-layers.md): mesh layers.
- [Particle systems](editor/user-properties-and-presets-particle-systems.md): particle systems.
- [Lights](editor/user-properties-and-presets-lights.md): lights.
- [Models and 3D](editor/user-properties-and-presets-models-and-3d.md): models and 3d.
- [Editor frame](editor/editor-frame.md): editor frame.
- [Layouts](editor/editor-layouts.md): the six layouts, their trees and what is saved.
- [Editor panels](editor/editor-panels.md): each panel of the workbench.
- [Preview](editor/preview.md): preview.
- [Pointer areas and sound bindings and Shared parts](editor/pointer-areas-and-sound-bindings.md): pointer areas and sound bindings, shared parts.
- [Web pages and other packages](editor/web-pages-and-other-packages.md): web pages and other packages.
- [Preset packages](editor/web-pages-and-other-packages-preset-packages.md): preset packages.
- [Starting points, building and applying](editor/starting-points-building-and-applying.md): starting points, building and applying.
- [Picture depth tools](editor/picture-depth-tools.md): picture depth tools.
- [Editor keyboard and non-scene packages and The own transition of a scene](editor/editor-keyboard-and-non-scene-packages.md): editor keyboard and non-scene packages, the own transition of a scene.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="document"></a>[Document](editor/document.md#document)
- <a id="operations"></a>[Operations](editor/document.md#operations)
- <a id="actions"></a>[Actions](editor/actions.md#actions)
- <a id="curve-editor"></a>[Curve editor](editor/actions-curve-editor.md#curve-editor)
- <a id="actions-from-the-command-line"></a>[Actions from the command line](editor/actions-from-the-command-line.md#actions-from-the-command-line)
- <a id="scripts-and-templates"></a>[Scripts and templates](editor/actions-from-the-command-line.md#scripts-and-templates)
- <a id="package-effects-and-shaders"></a>[Package effects and shaders](editor/package-effects-and-shaders.md#package-effects-and-shaders)
- <a id="history"></a>[History](editor/package-effects-and-shaders.md#history)
- <a id="inspector"></a>[Inspector](editor/inspector.md#inspector)
- <a id="shapes-text-and-fonts"></a>[Shapes, text and fonts](editor/inspector-shapes-text-and-fonts.md#shapes-text-and-fonts)
- <a id="hearing-shaping-and-trying-a-binding"></a>[Hearing, shaping and trying a binding](editor/inspector-shapes-text-and-fonts.md#hearing-shaping-and-trying-a-binding)
- <a id="sound-layers"></a>[Sound layers](editor/inspector-shapes-text-and-fonts.md#sound-layers)
- <a id="package-details"></a>[Package details](editor/package-details.md#package-details)
- <a id="user-properties-and-presets"></a>[User properties and presets](editor/user-properties-and-presets.md#user-properties-and-presets)
- <a id="mesh-layers"></a>[Mesh layers](editor/user-properties-and-presets-mesh-layers.md#mesh-layers)
- <a id="particle-systems"></a>[Particle systems](editor/user-properties-and-presets-particle-systems.md#particle-systems)
- <a id="lights"></a>[Lights](editor/user-properties-and-presets-lights.md#lights)
- <a id="models-and-3d"></a>[Models and 3D](editor/user-properties-and-presets-models-and-3d.md#models-and-3d)
- <a id="editor-frame"></a>[Editor frame](editor/editor-frame.md#editor-frame)
- <a id="canvas-bar"></a>[Canvas bar](editor/editor-panels.md#canvas-bar)
- <a id="right-side"></a>[Right side](editor/editor-panels.md#right-side)
- <a id="problems"></a>[Problems](editor/editor-panels.md#problems)
- <a id="timeline"></a>[Timeline](editor/editor-panels.md#timeline)
- <a id="preview"></a>[Preview](editor/preview.md#preview)
- <a id="pointer-areas-and-sound-bindings"></a>[Pointer areas and sound bindings](editor/pointer-areas-and-sound-bindings.md#pointer-areas-and-sound-bindings)
- <a id="shared-parts"></a>[Shared parts](editor/pointer-areas-and-sound-bindings.md#shared-parts)
- <a id="web-pages-and-other-packages"></a>[Web pages and other packages](editor/web-pages-and-other-packages.md#web-pages-and-other-packages)
- <a id="preset-packages"></a>[Preset packages](editor/web-pages-and-other-packages-preset-packages.md#preset-packages)
- <a id="starting-points-building-and-applying"></a>[Starting points, building and applying](editor/starting-points-building-and-applying.md#starting-points-building-and-applying)
- <a id="picture-depth-tools"></a>[Picture depth tools](editor/picture-depth-tools.md#picture-depth-tools)
- <a id="editor-keyboard-and-non-scene-packages"></a>[Editor keyboard and non-scene packages](editor/editor-keyboard-and-non-scene-packages.md#editor-keyboard-and-non-scene-packages)
- <a id="the-own-transition-of-a-scene"></a>[The own transition of a scene](editor/editor-keyboard-and-non-scene-packages.md#the-own-transition-of-a-scene)
