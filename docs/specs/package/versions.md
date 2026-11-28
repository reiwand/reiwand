# Versions and Manifest

On this page: versions, manifest.

## Versions

`schema` is `reiwand.wpak/3`. Every reader (daemon, library, repositories, server and the Android app)
refuses an older manifest with `<schema> is no longer read: this version reads only reiwand.wpak/3, so the
package has to be made again in that format and signed again`. Within `/3` only new feature names and new
enum values are added, and the meaning of what exists never changes. A field that is added to something
that exists comes with a new major version of the format, and no program reads or converts the older one.

Every value is written. A manifest that lacks a field is refused naming it (`manifest.json: "requires.engine"
is missing. Reiwand does not fill in values: write it.`); nothing is
completed silently. The only fields that may be absent are the unused optional blocks (`content`, `display`,
`links`, `origin`, `permissions`, `playback`, `previews`, `requires`, `web`, and inside a property `group`,
`help`, `icon`, `label`, `unit`, `default_asset`): an absent block and `null` mean the same, but a block that
is present carries all its fields. A value whose `null` has its own meaning (`playback.end`, `playback.max_fps`,
`display.accent`) is always written. The editor writes whole documents with the prefilled values it shows;
the table of every field, its class and its prefilled value is `crates/reiwand-format/src/prefill.tsv`.
`reiwand-ctl pack build` and the server refuse an incomplete manifest naming the field, and a package
without `content` and `origin`, which they require although the format lets them be absent.

Readers work in two steps. They first read a tolerant header (`schema` and `requires`): a newer
major version, a `requires.engine` that this version of Reiwand does not meet, or a name in
`requires.features` that it does not know returns `Newer` (`this package needs a newer version of
Reiwand: it uses …`) instead of an error about an unknown field. Only then is the manifest
read strictly, refusing unknown fields.

In the desktop library a package that needs a newer version still shows its thumbnail
(`previews.thumbnail`, read without the strict manifest) next to the reason. The
library remembers which version of Reiwand read each package and reads every package again once
after an upgrade, so packages that needed a newer version start working and older ones get the
facets of the newer rules.

## Manifest

| Field | Rules |
|-------|-------|
| `schema`, `id`, `version`, `kind`, `entry` | `id` is a lowercase reverse-DNS name (`org.example.dusk`), `version` SemVer |
| `title` | 1 to 128 characters on one line; also `{language: text}` or `"@key"` |
| `description` | text, map or `"@key"` of at most 8000 characters, with the markup below; `""` when there is none |
| `changes` | notes of this version, at most 2000 characters, with the markup below; `null` when there are none |
| `default_locale`, `strings` | see "Languages"; `strings` is `{}` when there are none |
| `authors`, `license`, `tags` | at most 64 authors of 1 to 128 characters, an SPDX expression of at most 256 characters, at most 32 tags of 1 to 32 characters; authors and tags have no `<` or `>`; `[]` when there are none |
| `platforms` | `[desktop, android]`, each once: the devices the package is made for, see "Platforms"; `[]` when it is made for all |
| `content` | `{rating: general \| mature, descriptors: [sexual, violence, gore, language]}`; `mature` names at least one reason, each once; required by `pack build` |
| `photosensitive` | `true` when the package flashes or shows fast patterns that can trigger seizures |
| `origin` | `{kind: original}` for a work made for Reiwand, `{kind: derived, derived_from: [{author, title, url, license}]}` for one remade by hand from 1 to 64 other works, crediting their authors (authors and titles without `<` or `>`), `{kind: imported}` for a conversion kept for personal use, or `{kind: provider, provider: {name, item_url, credit, terms_url}}` for a picture or video downloaded from an online source; every `origin` also writes `source_url` (`null` when there is none) and `derived_from` and `provider` as `[]` and `null` when the kind does not use them, see "Origin"; required by `pack build` |
| `credits` | at most 64 `{paths, license, attribution, url, source}`: who made which files (a path ending in `/` names a folder) under which SPDX license, and `source: {id, version}` when they were copied from an `assets` package (`null` otherwise); paths and attribution have no `<` or `>` |
| `links` | `{homepage, source, support, donate}`, each `https://` and at most 512 characters, or `null` |
| `previews` | `{thumbnail, animated}`: png, jpg or webp of at most 1024 px a side and 1 MiB; gif or webp of at most 640 px, 15 s and 8 MiB; the file must be what its extension says; each `null` when there is none |
| `requires` | `{engine: "<SemVer requirement>", features: [..], packages: [{id, version}], gpu: any \| compute \| null}`, see "Features" and "Dependencies" |
| `inputs` | `[audio, pointer, clicks, media, network]`, see "Inputs"; `[]` when there are none |
| `permissions` | `{network: {allow: [..]}, rgb}`, `web` and `scene`: `network` for web pages only, see "Web pages"; `rgb` see "RGB lighting" |
| `web` | `{preload, background, storage_kib}`, `web` only, see "Web pages" |
| `playback` | `video`, `web` and `scene`: how the content plays when the user has not chosen, see "Display and playback" |
| `display` | how a picture or video is placed and the accent it suggests, see "Display and playback" |
| `properties`, `presets` | user properties (`scene.md`): at most 256 with ids `[A-Za-z][A-Za-z0-9_]{0,63}` that differ in more than case, combos of at most 64 options, files and folders that accept `image`, `video` or both; at most 64 presets for scenes and videos; `{}` and `[]` when there are none |

The whole manifest of a small scene (`docs/authoring/examples/first-scene`, written by `reiwand-ctl scene new` and the editor; every unused block is `null` here and may also be left out):

```json
{
  "authors": [],
  "changes": null,
  "content": {
    "descriptors": [],
    "rating": "general"
  },
  "credits": [],
  "default_locale": "en",
  "description": "",
  "display": null,
  "entry": "scene.json",
  "id": "org.example.first-scene",
  "inputs": [],
  "kind": "scene",
  "license": "CC0-1.0",
  "links": null,
  "origin": {
    "derived_from": [],
    "kind": "original",
    "provider": null,
    "source_url": null
  },
  "permissions": null,
  "photosensitive": false,
  "platforms": [],
  "playback": null,
  "presets": [],
  "previews": null,
  "properties": {
    "color_sun": {
      "alpha": false,
      "default": "#ffd23f",
      "enabled_if": {},
      "label": "Sun color",
      "order": 1,
      "quality": {},
      "type": "color",
      "visible_if": {}
    }
  },
  "requires": null,
  "schema": "reiwand.wpak/3",
  "strings": {},
  "tags": [],
  "title": "My first scene",
  "version": "0.1.0",
  "web": null
}
```
