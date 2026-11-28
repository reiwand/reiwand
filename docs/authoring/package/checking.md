# Previews and checking

On this page: previews and what the check looks at.

## Previews

`previews.thumbnail` is the image seen in libraries. `previews.animated` is optional.

| Field | Format | Limits |
|-------|--------|--------|
| `thumbnail` | png, jpg or webp | up to 1024 pixels per side and 1 MiB; the file must be what its extension says |
| `animated` | gif or webp | up to 640 pixels per side, 15 s and 8 MiB |

Three ways to set them, all inside the folder:

```
reiwand-ctl scene thumbnail my-scene capture.png
reiwand-ctl scene thumbnail my-scene --capture --time 3
reiwand-ctl scene thumbnail my-scene --capture --animated --seconds 4 --fps 10
```

The first copies an image you already have to `assets/preview.png` (it shrinks it if it goes over
1024 pixels or 1 MiB) and writes `previews.thumbnail`. The other two draw the scene with the
computer's graphics card and require the daemon to be running. `--time` is the second of the scene
that is saved (without it, the second of its still frame, `clock.poster_time`).

**`scene render` runs the scripts, the daemon's capture does not.** `scene render` runs the scripts
of the folder (in the separate sandboxed process) with this computer's time (or that of
`--time-of-day`) and lets you see the result; whereas `scene thumbnail --capture` draws the scene
as `scene.json` leaves it: a text that a script fills in comes out with its initial value. Put
starting values in `scene.json` that look good, or use an image made with `scene render`.

`reiwand-ctl scene render my-scene -o view.png --size 640` draws an image without a desktop,
without the daemon and without saving anything in the folder; it is useful to see the result, to
make clips (`--frames`) and to compare quality levels (`--against q1`).

## Checking the package

There are three check commands. None needs the daemon.

| Command | What for |
|---------|----------|
| `reiwand-ctl pack check FOLDER` | Lists **all** the problems of the scene and the manifest, each with the JSON Pointer of the field (`/manifest/inputs`, `/scene/layers/3`). It exits with 1 if there are errors. `--json` gives one object per problem. |
| `reiwand-ctl pack check FOLDER_OR_FILE` | Summarizes what the package needs, uses and declares, and whether the desktop and the phone play it. It stops at the first error. |
| `reiwand-ctl pack validate FOLDER_OR_FILE --key author.pub` | Checks that the package is valid and, with `--key`, that it was signed by that key. |

A healthy package:

```
$ reiwand-ctl pack check docs/authoring/examples/follow-pointer
ok scene org.example.follow-pointer 1.0.0 (reiwand.wpak/3)
title = Pointer follower
rating = general
photosensitive = no
origin = original
inputs = pointer
needs = inputs.pointer (follows the pointer), scripts (runs scene scripts)
build adds = inputs.pointer (follows the pointer), scripts (runs scene scripts)
facets = 1080p, android, animated, desktop, landscape, pointer, scripts
desktop = plays
android = plays
languages = en
```

| Line | What it says |
|------|--------------|
| `ok scene …` | The type, the id, the version and the schema. If there is an error, `Error: …` comes out instead. |
| `title`, `rating`, `photosensitive`, `origin` | What the manifest declares. |
| `inputs` | The inputs it declares. |
| `needs` | The engine functions the package uses. A version of Reiwand that lacks them rejects the package saying it needs a newer one. |
| `build adds` | Those of `needs` that are missing from `requires.features` of the manifest. `pack build` writes them by itself. |
| `facets` | The tags the program computes from the content (inputs, size, orientation, `scripts`, `clock`, `customizable`…). They cannot be faked: they come from the package, not from what you say. They are for searching (`facet:audio`). |
| `desktop`, `android` | `plays`, `plays without …` (it plays it without some function) or `refuses …` (it rejects it, with the reason). |
| `languages` | The languages of the texts. |

The check of a folder tells **errors** (they keep the package from being built) from **warnings**.
The most common ones:

| Message | What it means and how to fix it |
|---------|---------------------------------|
| `inputs: the scene uses pointer at /scripts/0 of scene.json; add "pointer"` (error) | The scene uses an input that the manifest does not declare. Add it to `inputs`. |
| `inputs declares audio but nothing in the package uses it` (warning) | You declare an input that nothing uses. Remove it: otherwise the user sees a permission that is not needed. |
| `requires.features: add "scripts" …; pack build fills this list` (warning) | A function is missing from `requires.features`. No need to fix it by hand: `pack build` and `pack sign` fill it in. |
| `missing file "missing.png"` (error) | The manifest or the scene names a file that is not in the folder. Put it there, or remove the reference. |
| `previews.thumbnail: "big.png" has 3000000 bytes, more than the 1048576 allowed` (error) | The thumbnail goes over 1 MiB or 1024 pixels per side. Shrink it with `scene thumbnail`. |
| `content: is required: a rating of general or mature and its reasons` (error) | `content` is missing. See "Rating". |
| `photosensitive: is required: true when the package flashes` (error) | It is not said whether the package flashes. |
| `origin: is required: original, or derived with the works it comes from` / `origin.derived_from: derived packages credit 1 to 64 works…` (error) | `origin` is missing, or it is `derived` without the works it comes from. |
| `version: expected a semantic version` (error) | Write `1.0.0`, not `one`. |
| `id: expected a lowercase reverse-DNS name like org.example.name` (error) | The id has capitals or no dots. |
| `license: expected an SPDX license expression …` (error) | The license is empty or badly written. |
| `default_locale: is required with strings or localized texts` (error) | There are texts per language and you do not say which one is the fallback. |
| `/scene/assets: extra.png is not used by the scene but goes into the package` (warning) | A file that no part of the scene uses. It is packaged anyway and takes space: delete it. |
| `/manifest: manifest.json.minisig signs an earlier version; sign again or delete it` (warning) | You changed something after signing. Sign again (see "Signing"). |
| `/scene/scripts/0/path: scripts/clock.js:4: unexpected token in expression: ';'` (error) | A script does not compile. The line is where the engine noticed, sometimes one after the typo. The same goes for `scripts/missing.js is listed in scripts but is not in the folder`: put the file there or remove the entry. |
| `/scene/layers/3: data did not match any variant of untagged enum Value` (error) | A layer has a value that is of no allowed type. The number is the position of the layer: look there for its misspelled field. |
| A package over 1 GiB, or with an image over 8192 pixels per side or 32 megapixels | It loses the `android` tag and `pack check` says why. |

None of the three runs the scripts, but `pack check` and `pack check` compile them: a syntax error
or a missing script comes out as a problem, with the file and the line, and makes the command fail
(`scripts/clock.js:4: unexpected token in expression: ';'`). Nor do they open the images of the
scene to see whether they look right. What they see and what they do not: `scripts.md`, "What
`validate` sees and what it does not".
