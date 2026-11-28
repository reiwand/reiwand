# Lights

The desktop and the phone draw light layers, volumetric light, projected textures and lit
particles. A scene without lights looks exactly as before.

## Light layers

Light layers (`type: light`) can be:

- point;
- spotlights, with their inner and outer cone;
- tubes;
- directional.

They have color, intensity, radius and height over the canvas, and are parented, attached to bones
and animated like other layers.

## Lit layers

Lights are not drawn themselves. Image, video and text layers with `lighting` receive them with:

- their normal map;
- roughness (with its map);
- reflection of an image or of what is underneath;
- own light (with its map);
- an intensity that blends the lit with the usual.

They also receive the ambient light of `canvas.ambient`, and that of the sky of `canvas.sky`
according to where the normal faces. Scripts read and change both like the background.

## Performance

Light is computed in the same pass that draws the layer, with 12 lights at most (8 on phones and
modest computers) and only as far as each reaches.

- With still lights the lit layers are drawn once and stored.
- A moving light only redraws the box of its radius.
- At the lowest quality, light is computed once at the start or when the level or size changes, and
  moving lights stay where they began, without asking for frames.

## Volumetric light

Point lights and spotlights can light the air (`volumetric`, with density, samples and shafts). It
is a glow that comes out of the light up to its radius, at the place of the light among the layers:
those on top cover it. The dark parts of the layers below cut the light into shafts if asked.

| Quality | Resolution |
| --- | --- |
| Otherwise | Half the screen size |
| Q1 and phones | A quarter |
| Q0 | Off |

It is stored while the light and what is under it are still.

## Projected textures

A spotlight projects an image, a video or a layer with its effects (`texture`, up to four
spotlights per scene) onto the lit layers and the air. Particles take only its color.

> **Note:** A layer that receives light cannot be projected, to avoid a loop.

## Lit and refracting particles

Particles receive the lights (`render.lit`) on both faces, up to 8, with a normal map that turns
with each particle (`render.normal`). This is the same in the GPU simulation as in the processor
one.

Particles can refract what is behind them with a normal map (`render.refract`, like drops on glass):

| Quality | Refraction |
| --- | --- |
| Q3 | Full size |
| Q2 | Half size |
| Q1 and Q0 | None |
