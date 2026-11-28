# The manifest

On this page: every field of `manifest.json`.

## The manifest

`manifest.json` is at the root of the package. Creating the scene with `reiwand-ctl scene new` (or
from the *Scenes* page of the app) already writes a valid one:

```
reiwand-ctl scene new my-scene --from clock --title "My clock" --locale en
```

It leaves `manifest.json`, `scene.json` and `assets/` in the folder. You can edit it with any text
editor; the app's editor has a dialog for each field (*Title, author, license and tags*).

## All the fields

Every field is written in `reiwand.json`. In the table, "no" means the field can be `null` (or empty), not that it can be left out: a missing field is refused.

| Field | Type | Required | What it is |
|-------|------|----------|------------|
| `schema` | text | yes | `reiwand.wpak/3`. A package of another version is rejected with a message that says so. |
| `id` | text | yes | A unique lowercase name, with dots, like `org.example.clock`. Each part is lowercase letters, digits, `_` or `-`. Do not change it between versions. |
| `version` | text | yes | A semantic version: `1.0.0`, `1.2.3`. Raise the version on every release. |
| `kind` | text | yes | `scene`, `web`, `image`, `video`, `preset` or `assets`. |
| `entry` | text | yes | The file it starts with: `scene.json`, `index.html`… |
| `title` | text or map | yes | From 1 to 128 characters on one line. It can be `{ "en": "Dusk", "de": "Abenddämmerung" }` or `"@key"`. |
| `description` | text, map or `"@key"` | no | Up to 8000 characters, with the format below. |
| `changes` | text | no | The notes of this version, up to 2000 characters, with the same format. |
| `default_locale` | text | yes, with texts in several languages | The fallback language, like `en` or `de`. Each text must have a value in it. |
| `strings` | object | no | The texts by language and key: `{ "en": { "speed": "Speed" } }`. They are used with `"@speed"`. Up to 64 languages and 2048 keys each. |
| `authors` | list of texts | no | Up to 64 names of 1 to 128 characters (without `<` or `>`). |
| `license` | text | yes | An SPDX expression: `CC0-1.0`, `CC-BY-4.0`, `MIT`, `CC-BY-SA-4.0 OR MIT`. See "Licenses". |
| `tags` | list of texts | no | Up to 32 tags of up to 32 characters. Free-form; for genres these are recommended: `abstract`, `anime`, `animals`, `cars`, `cityscape`, `fantasy`, `games`, `minimal`, `mountains`, `music`, `nature`, `pixel-art`, `retro`, `sci-fi`, `space`, `water`, `winter`. |
| `platforms` | list | no | `desktop`, `android` or both: which devices it is for. It only narrows: it never makes a package run where it cannot. |
| `content` | object | yes | `{ "rating": "general" }` or `{ "rating": "mature", "descriptors": ["violence"] }`. See "Rating". |
| `photosensitive` | `true` or `false` | yes | `true` if the package flashes or shows fast patterns that can trigger seizures. |
| `origin` | object | yes | Where the work comes from. See "Origin". |
| `credits` | list | no | Up to 64 `{ paths, license, attribution, url?, source? }`: who made which files and under what license. A path ending in `/` is a folder. |
| `links` | object | no | `{ homepage, source, support, donate }`, each optional, with `https://` and up to 512 characters. |
| `previews` | object | no | `{ thumbnail, animated }`. See "Previews". |
| `requires` | object | no | `{ engine, features, packages, gpu }`: what the package needs. `pack build` fills in `features` by itself. `packages` lists up to 16 `{ id, version }` of other packages that are installed with it. `gpu` is `any` or `compute`. |
| `inputs` | list | no, but the scene may require it | `audio`, `pointer`, `clicks`, `media` or `network`: what the package asks of the user. Declaring it does not grant it: the user allows it per package. See "Inputs". |
| `permissions` | object | no | `{ network: { allow: [...] }, rgb }`: the network origins of a web page and the permission to send colors to the RGB lighting. In a web page it is set with the checkbox "Sends its colors to the RGB lighting" among its inputs; in a scene it is set and removed by itself according to the layers with `rgb`. |
| `web` | object | only in `kind: web` | `{ preload, background, storage_kib }`. See `web-wallpapers.md`. |
| `playback` | object | no | How it plays by default: `speed`, `start`, `end`, `loop`, `max_fps`, `volume`, `muted`. For videos, pages and scenes. |
| `display` | object | no | How an image or a video is placed (`fit`, `background`, `span`, `flip`, `zoom`, `position`) and the accent color it suggests (`accent`). |
| `properties` | object | no | The properties the user changes (`properties.md`). |
| `presets` | list | no | Already chosen values of the properties, with a name (`properties.md`). |

A complete manifest, for the scene of [`examples/canvas-and-clock`](../examples/canvas-and-clock):

<!-- docs-examples: examples/canvas-and-clock/manifest.json -->
```json
{
  "schema": "reiwand.wpak/3",
  "id": "org.example.dusk",
  "version": "1.3.0",
  "kind": "scene",
  "entry": "scene.json",
  "default_locale": "en",
  "title": { "en": "Dusk", "de": "Abenddämmerung" },
  "description": { "en": "Hills at **dusk**.", "de": "Hügel in der **Abenddämmerung**." },
  "changes": null,
  "strings": {},
  "authors": ["Example"],
  "credits": [],
  "license": "CC-BY-4.0",
  "tags": ["nature", "minimal"],
  "platforms": [],
  "origin": { "kind": "original", "derived_from": [], "provider": null, "source_url": null },
  "content": { "rating": "general", "descriptors": [] },
  "photosensitive": false,
  "previews": { "thumbnail": "assets/sky.png", "animated": null },
  "links": { "homepage": null, "source": "https://example.org/dusk", "support": null, "donate": null },
  "inputs": ["pointer"],
  "requires": {
    "engine": null, "gpu": null, "packages": [],
    "features": ["bloom", "camera", "clock_dates", "inputs.pointer", "parallax2", "texture_options"]
  },
  "properties": {},
  "presets": []
}
```

The texts (`description`, `changes`) allow a minimal and safe format: paragraphs separated by an
empty line, lists with `- ` or `* `, `**bold**`, `*italic*` and links `[text](https://…)`. HTML,
images and links that are not `https://` are not allowed. The other texts (titles, labels, options)
are plain text.
