# The package manifest

The manifest of a package describes it. This page lists what a `reiwand.wpak/3` manifest declares,
which texts can come in several languages, and which kinds of package the format describes.
`docs/specs/package.md` describes the whole format.

## What the manifest declares

In `reiwand.wpak/3` the manifest declares:

| Item | What it says |
| --- | --- |
| Rating | For all audiences or for adults, with its reasons. |
| Photosensitivity | Whether it flashes or shows fast patterns that can trigger seizures in people with photosensitive epilepsy. |
| Origin | Whether it is an original work or one remade by hand from others, with the name of its authors. |
| Credits and license | The credits and the license of each file. |
| Inputs | The inputs it can ask for: sound, pointer, clicks, media and network. |
| Links | Its links. |
| Requirements | What it needs from the engine. |

> **Note:** Packages of the first manifest version are no longer opened.

## Languages

The title, the description, the change notes and the texts of its properties can come in several
languages, with string tables. Texts can also come in one file per language
(`locale/<language>.json`).

Reiwand picks a language with a fallback chain:

1. Your language.
2. Its base language.
3. The package's language.

The scripts of a scene receive the chosen language.

The six example scenes are described in English and in Spanish, and `pulse` warns that it flashes
with the beat.

## Formatted text

The description and the notes allow paragraphs, lists, bold, italics and `https` links. The app
opens a link only with a click and after you confirm it.

The rest of the texts are shown as they are and cannot carry `<` or `>`.

## Kinds of package

Besides images, videos and scenes, the format describes:

| Kind | Description |
| --- | --- |
| Web pages (`web`) | An entry page, a script that loads before its own, local storage of 5 MiB at most that is never synchronized, and up to 8 `https` origins it can reach if you give network permission. The `window.reiwand` API lets it receive settings, sound, pause and folders. |
| Presets published separately (`preset`) | They depend on a base package by id and version range and only carry values and their images or videos. |
| Resource packages (`assets`) | Other authors copy them into their scenes, with credits that name the source package. |
| Imported packages | Imported for personal use; never published. |
| Provider packages | Downloaded from a provider, with their credit and a free license. |

See [Assets, presets and dependencies](assets-and-presets.md) for assets and presets.

## Other manifest fields

The format also describes:

- dependencies between packages (`requires.packages`);
- the devices it is made for (`platforms`);
- the accent color it proposes to the desktop;
- the color around the image;
- spanning over all screens;
- the flip;
- the frame limit;
- the silent start;
- the single playback.

## Features and platforms

Each feature has a name in `requires.features`. The desktop and the phone say, for each feature,
whether they:

- run it;
- play the package without it; or
- reject it with "needs a newer version of Reiwand" before drawing anything.

In this version they reject separately published resources and AVIF images.

`reiwand-ctl pack check` says what each platform does with a package. See
[Compatibility and versions](compatibility.md) for more.
