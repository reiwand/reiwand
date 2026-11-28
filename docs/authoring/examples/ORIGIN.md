# Origin of the example files

`hum.wav` (in `animation-and-interaction/assets` and `sound/assets`) is a synthesized tone and
`apps/reiwand-app/look/pictures/example.jpg` is a picture drawn with code. Both are produced by
`scripts/example-assets.py`, so they belong to the project and carry the same license as the
examples (CC0-1.0, "Reiwand contributors").

The pictures, the color table (`teal-orange.cube`) and the green-screen video (`balloon.webm`) of
`blend-modes`, `canvas-extras`, `effects-color`, `effects-distort`, `effects-light`,
`effects-reveal`, `timelines` and `values-extras` are drawn with code by
`scripts/showcase-assets.py` (the video also needs `ffmpeg`), under the same license.

The other pictures, models and the video were drawn with code when the examples were written, and
the code that drew them was not kept; they hold nothing taken from elsewhere:

- `sky.png` (the same file in `canvas-and-clock`, `depth`, `first-scene`, `groups-and-masks`,
  `mesh-with-bones`, `own-effect`, `properties` and `stacked-effects`): a blue gradient with white
  discs.
- `depth/assets/depth.png`: a vertical grey ramp used as a depth map.
- `lights/assets/stone.png` and `relief.png`: blurred brown noise and a diagonal normal-map pattern.
- `stacked-effects/assets/stain.png`: a radial white-to-black blur.
- `model-3d/assets/figure.glb`, `box.glb` and `mesh-with-bones/assets/doll.glb`: a few meshes each,
  written as glTF by code, with no modelling tool.
- `video/assets/frames.webm`: one second of 64×32 frames encoded with `ffmpeg`.
