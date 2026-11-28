# Scene format

A scene package is a `.wpak` package (see `package.md` for packages and their manifest, `repo.md`
for repositories) whose manifest has `"kind": "scene"` and `"entry": "scene.json"`. The scene
lists its assets by package path; every path must be in the package. `reiwand-ctl pack build` and
every reader check the scene: unknown fields, broken references and out-of-range values are refused
with the JSON path of the field, such as `/layers/2/effects/0/params/radius is 300, expected
0..=128`. The model lives in
`crates/reiwand-scene`; `reiwand-ctl scene bench <file.wpak>` renders a package offscreen and reports
what each frame costs, how long the scene takes to load and draw its first frame, and the GPU and
main memory it holds (`--screens 2` shows it on two screens at once; `--passes` times each render
pass with GPU timestamps, where the GPU has them inside command encoders, every 15th frame, and lists
the passes from the dearest, the passes of each 3D group among them (`3d shadow maps`, `3d depth for
particles`, `3d models` and `3d second camera and cross-fade`, with the size of the group's picture), as the performance report of the phone does; `--shot frame.png` saves
the last frame, so two builds can be compared picture by picture; `--quality auto` steers the level
by measured GPU time as the phone does, starting where the phone would and listing every step;
`--paced 30` renders in real time at `--fps` for 30 seconds, with at most two frames on the GPU like
the swapchain of the phone, and reports the rate it held and the slots it missed; it does not time
the particle steps apart, so the CPU never waits for the GPU there). The same bench runs on a phone:
`scripts/phone-bench.sh build` cross-compiles the `bench` example of `crates/reiwand-android` for arm64
(or x86_64 for an emulator with `REIWAND_ABI=x86_64`) with the NDK and the same path remaps as the Gradle build of the app, so both share one cache, `copy FILE.wpak...` copies it and the packages to
`/data/local/tmp/reiwand-bench` (nothing else on the phone is touched and no app is installed), and
`run NAME.wpak --level q3 --passes` or `--level auto --paced 30` (with `--pace smooth`, the
automatic level never halves the rate; `detail`, the default, as `playback.auto_pace`) loads the package the way the
live wallpaper does (phone limits, properties, tier from its GPU, 1440x3040 unless `--size` says
otherwise) and prints the report as JSON with the battery temperature and thermal status before
and after. The serial of the phone comes from `REIWAND_ADB_SERIAL`; each run waits while the screen is
on (five-minute checks, `REIWAND_AWAKE_TRIES` of them) and until the battery is under `REIWAND_COOL`
degrees (36 when the variable is not set).

The specification, part by part:

- [Every value is written](scene/every-value-is-written.md): every value is written.
- [Canvas, clock and assets](scene/canvas-clock-and-assets.md): canvas, clock and assets.
- [KTX2 textures](scene/canvas-clock-and-assets-ktx2-textures.md): ktx2 textures.
- [Cost and frame rate](scene/cost-and-frame-rate.md): cost and frame rate.
- [Layers](scene/layers.md): layers.
- [Text](scene/layers-text.md): text.
- [Lights](scene/layers-lights.md): lights.
- [Meshes](scene/layers-meshes.md): meshes.
- [Models and 3D](scene/layers-models-and-3d.md): models and 3d.
- [Values](scene/values.md): values.
- [Effects](scene/effects.md): effects.
- [Writing an effect](scene/effects-writing-an-effect.md): writing an effect.
- [Package effects](scene/effects-package-effects.md): package effects.
- [Package transitions](scene/effects-package-transitions.md): package transitions.
- [Newer effects and parameters](scene/effects-newer-effects-and-parame.md): newer effects and parameters.
- [Sound](scene/sound.md): sound.
- [Particles](scene/particles.md): particles.
- [Newer particle fields](scene/particles-newer-particle-fields.md): newer particle fields.
- [User properties and presets](scene/user-properties-and-presets.md): user properties and presets.
- [Inputs](scene/inputs.md): inputs.
- [Scripts](scene/scripts.md): lifecycle.
- [The `reiwand` module](scene/scripts-the-reiwand-module.md): the `reiwand` module.
- [Parents, the camera, shapes, storage and folders](scene/scripts-parents-the-camera-shape.md): parents, the camera, shapes, storage and folders.
- [When a script fails, Listening to the audio from a script and Script declarations](scene/scripts-when-a-script-fails.md): when a script fails, listening to the audio from a script, script declarations.
- [Rebuilding a scene by hand](scene/rebuilding-a-scene-by-hand.md): effects with an equivalent.
- [Porting a shader to WGSL](scene/rebuilding-a-scene-by-hand-porting-a-shader-to-wgsl.md): porting a shader to wgsl.

## Where the sections went

The sections of this page moved to the pages above; links to them still land here:

- <a id="every-value-is-written"></a>[Every value is written](scene/every-value-is-written.md#every-value-is-written)
- <a id="canvas-clock-and-assets"></a>[Canvas, clock and assets](scene/canvas-clock-and-assets.md#canvas-clock-and-assets)
- <a id="ktx2-textures"></a>[KTX2 textures](scene/canvas-clock-and-assets-ktx2-textures.md#ktx2-textures)
- <a id="cost-and-frame-rate"></a>[Cost and frame rate](scene/cost-and-frame-rate.md#cost-and-frame-rate)
- <a id="layers"></a>[Layers](scene/layers.md#layers)
- <a id="text"></a>[Text](scene/layers-text.md#text)
- <a id="lights"></a>[Lights](scene/layers-lights.md#lights)
- <a id="meshes"></a>[Meshes](scene/layers-meshes.md#meshes)
- <a id="models-and-3d"></a>[Models and 3D](scene/layers-models-and-3d.md#models-and-3d)
- <a id="values"></a>[Values](scene/values.md#values)
- <a id="effects"></a>[Effects](scene/effects.md#effects)
- <a id="writing-an-effect"></a>[Writing an effect](scene/effects-writing-an-effect.md#writing-an-effect)
- <a id="package-effects"></a>[Package effects](scene/effects-package-effects.md#package-effects)
- <a id="package-transitions"></a>[Package transitions](scene/effects-package-transitions.md#package-transitions)
- <a id="newer-effects-and-parameters"></a>[Newer effects and parameters](scene/effects-newer-effects-and-parame.md#newer-effects-and-parameters)
- <a id="sound"></a>[Sound](scene/sound.md#sound)
- <a id="particles"></a>[Particles](scene/particles.md#particles)
- <a id="newer-particle-fields"></a>[Newer particle fields](scene/particles-newer-particle-fields.md#newer-particle-fields)
- <a id="user-properties-and-presets"></a>[User properties and presets](scene/user-properties-and-presets.md#user-properties-and-presets)
- <a id="inputs"></a>[Inputs](scene/inputs.md#inputs)
- <a id="scripts"></a>[Scripts](scene/scripts.md#scripts)
- <a id="lifecycle"></a>[Lifecycle](scene/scripts.md#lifecycle)
- <a id="the-reiwand-module"></a>[The `reiwand` module](scene/scripts-the-reiwand-module.md#the-reiwand-module)
- <a id="parents-the-camera-shapes-storage-and-folders"></a>[Parents, the camera, shapes, storage and folders](scene/scripts-parents-the-camera-shape.md#parents-the-camera-shapes-storage-and-folders)
- <a id="when-a-script-fails"></a>[When a script fails](scene/scripts-when-a-script-fails.md#when-a-script-fails)
- <a id="listening-to-the-audio-from-a-script"></a>[Listening to the audio from a script](scene/scripts-when-a-script-fails.md#listening-to-the-audio-from-a-script)
- <a id="script-declarations"></a>[Script declarations](scene/scripts-when-a-script-fails.md#script-declarations)
- <a id="rebuilding-a-scene-by-hand"></a>[Rebuilding a scene by hand](scene/rebuilding-a-scene-by-hand.md#rebuilding-a-scene-by-hand)
- <a id="effects-with-an-equivalent"></a>[Effects with an equivalent](scene/rebuilding-a-scene-by-hand.md#effects-with-an-equivalent)
- <a id="porting-a-shader-to-wgsl"></a>[Porting a shader to WGSL](scene/rebuilding-a-scene-by-hand-porting-a-shader-to-wgsl.md#porting-a-shader-to-wgsl)
